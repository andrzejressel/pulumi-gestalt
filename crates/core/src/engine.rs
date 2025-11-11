use crate::config::{Config, RawConfigValue};
use crate::{FunctionName, RawOutput, RegisterResourceOutput};
use futures::FutureExt;
use futures::stream::FuturesOrdered;
use pulumi_gestalt_domain::connector::{
    PulumiConnector, RegisterOutputsRequest, RegisterResourceRequest, ResourceInvokeRequest,
};
use pulumi_gestalt_domain::{ExistingNodeValue, FieldName, NodeValue};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::Sender;
use tokio::task::JoinSet;

pub struct NativeFunctionRequest {
    function_name: FunctionName,
    data: Value,
    return_mailbox: Sender<Value>,
}

pub enum ConfigValue {
    PlainText(String),
    Secret(RawOutput),
}

pub struct Engine {
    outputs: HashMap<FieldName, RawOutput>,
    join_set: JoinSet<()>,
    native_function_queue_sender: UnboundedSender<NativeFunctionRequest>,
    native_function_queue_receiver: UnboundedReceiver<NativeFunctionRequest>,

    output_task_created: AtomicBool,
    pulumi: Arc<Box<dyn PulumiConnector>>,
    config: Config,
}

static_assertions::assert_impl_all!(Engine: Send, Sync);

impl Engine {
    pub fn new(pulumi: impl PulumiConnector + 'static, config: Config) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        Self {
            outputs: HashMap::new(),
            join_set: JoinSet::new(),
            pulumi: Arc::new(Box::new(pulumi)),
            output_task_created: AtomicBool::new(false),
            native_function_queue_sender: tx,
            native_function_queue_receiver: rx,
            config,
        }
    }

    #[cfg(test)]
    pub fn new_without_configs(pulumi: impl PulumiConnector + 'static) -> Self {
        let config = Config::new(HashMap::new(), HashSet::new());
        Self::new(pulumi, config)
    }

    pub fn add_output(&mut self, field_name: FieldName, output_id: RawOutput) {
        self.outputs.insert(field_name, output_id);
    }

    pub fn create_register_resource_node(
        &mut self,
        r#type: String,
        name: String,
        inputs: HashMap<FieldName, RawOutput>,
        version: String,
    ) -> RegisterResourceOutput {
        let pulumi = self.pulumi.clone();
        let output = RegisterResourceOutput::from_future(async move {
            let mut resolved_inputs = HashMap::new();
            for (key, output) in inputs {
                let value = output.value.await;
                resolved_inputs.insert(key, value);
            }

            let result = pulumi
                .register_resource(
                    RegisterResourceRequest::builder()
                        .r#type(r#type)
                        .name(name)
                        .version(version)
                        .object(resolved_inputs)
                        .build(),
                )
                .await;

            Arc::new(result.fields)
        });
        self.join_set.spawn(output.clone().invoke_void());

        output
    }

    pub fn create_resource_invoke_node(
        &mut self,
        token: String,
        inputs: HashMap<FieldName, RawOutput>,
        version: String
    ) -> RegisterResourceOutput {
        let pulumi = self.pulumi.clone();
        let output = RegisterResourceOutput::from_future(async move {
            let mut resolved_inputs = HashMap::new();
            for (key, output) in inputs {
                let value = output.value.await;
                resolved_inputs.insert(key, value);
            }

            let result = pulumi
                .resource_invoke(
                    ResourceInvokeRequest::builder()
                        .token(token)
                        .version(version)
                        .object(resolved_inputs)
                        .build(),
                )
                .await;

            Arc::new(result.fields)
        });
        self.join_set.spawn(output.clone().invoke_void());

        output
    }

    pub fn create_native_function_node(
        &mut self,
        function_name: FunctionName,
        source: RawOutput,
    ) -> RawOutput {
        let request_receiver = self.native_function_queue_sender.clone();
        let output = RawOutput::from_future(async move {
            let source_value = source.value.await;
            match source_value {
                NodeValue::Nothing => NodeValue::Nothing,
                NodeValue::Exists(ExistingNodeValue { value, secret }) => {
                    let (tx, rx) = tokio::sync::oneshot::channel();
                    let request = NativeFunctionRequest {
                        function_name,
                        data: value,
                        return_mailbox: tx,
                    };
                    request_receiver.send(request).unwrap();

                    let result = rx.await.unwrap();
                    NodeValue::exists(result, secret)
                }
            }
        });
        self.join_set.spawn(output.clone().invoke_void());
        output
    }

    pub fn create_combine_outputs(&mut self, outputs: Vec<RawOutput>) -> RawOutput {
        use futures::StreamExt;
        RawOutput::from_future(async move {
            let mut combined = FuturesOrdered::new();
            for output in outputs {
                combined.push_back(output.value);
            }

            let results: Vec<_> = combined.collect().await;

            let mut combined = Vec::with_capacity(results.len());
            let secret = results.iter().any(|res| match res {
                NodeValue::Exists(ExistingNodeValue { secret, .. }) => *secret,
                NodeValue::Nothing => false,
            });

            for result in results {
                match result {
                    NodeValue::Nothing => {
                        return NodeValue::Nothing;
                    }
                    NodeValue::Exists(ExistingNodeValue { value, .. }) => {
                        combined.push(value);
                    }
                }
            }

            NodeValue::exists(Value::Array(combined), secret)
        })
    }

    pub fn create_done_node(&mut self, value: Value, secret: bool) -> RawOutput {
        let node_value = NodeValue::exists(value, secret);
        RawOutput::from_node_value(node_value)
    }
    
    pub fn create_extract_field(&mut self, field_name: FieldName, source: RegisterResourceOutput) -> RawOutput {
        let output = RawOutput::from_future(async move {
            let resource_fields = source.value.await;
            resource_fields.get_field_value(&field_name)
        });
        self.join_set.spawn(output.clone().invoke_void());
        output
    }

    #[cfg(test)]
    fn create_nothing_node(&mut self) -> RawOutput {
        RawOutput::from_node_value(NodeValue::Nothing)
    }

    async fn run(&mut self) -> Option<NativeFunctionRequest> {
        if self
            .output_task_created
            .compare_exchange(false, true, SeqCst, SeqCst)
            .is_ok()
        {
            let outputs = self.outputs.clone();
            let pulumi = self.pulumi.clone();

            let f = async move {
                let mut resolved_outputs = HashMap::new();
                for (key, output) in outputs {
                    let value = output.value.await;
                    resolved_outputs.insert(key, value);
                }

                pulumi
                    .register_outputs(
                        RegisterOutputsRequest::builder()
                            .outputs(resolved_outputs)
                            .build(),
                    )
                    .await;
            };

            self.join_set.spawn(f);
        }

        loop {
            tokio::select! {
                res = self.join_set.join_next() => {
                    match res {
                        Some(_) => {
                            continue
                        }
                        None => {
                            return None;
                        }
                    }
                }
                Some(request) = self.native_function_queue_receiver.recv() => {
                    return Some(request)
                }
            }
        }
    }

    pub fn get_config_value(&mut self, name: &str, key: &str) -> Option<ConfigValue> {
        match self.config.get(name, key) {
            None => None,
            Some(RawConfigValue::PlainText(value)) => Some(ConfigValue::PlainText(value.clone())),
            Some(RawConfigValue::Secret(secret)) => {
                let value = Value::String(secret.clone());
                let output_id = self.create_done_node(value, true);
                Some(ConfigValue::Secret(output_id))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use std::collections::HashMap;

    mod register_outputs {
        use super::*;
        use pulumi_gestalt_domain::connector::MockPulumiConnector;

        #[tokio::test]
        async fn does_not_register_outputs_when_none_added() {
            let mut mock = MockPulumiConnector::new();
            mock.expect_register_outputs().times(0);

            let mut engine = Engine::new_without_configs(mock);

            let result = engine.run().await;
        }

        #[tokio::test]
        async fn should_register_outputs() {
            let mut mock = MockPulumiConnector::new();
            mock.expect_register_outputs()
                .times(1)
                .with(eq(RegisterOutputsRequest::new(HashMap::from([(
                    "output".into(),
                    NodeValue::exists(1, false),
                )]))))
                .returning(|_| ());

            let mut engine = Engine::new_without_configs(mock);

            let output_id = engine.create_done_node(1.into(), false);
            engine.add_output("output".into(), output_id);

            let result = engine.run().await;
            assert!(result.is_none());
        }

        #[tokio::test]
        async fn should_only_create_output_task_once() {
            let mut mock = MockPulumiConnector::new();
            mock.expect_register_outputs()
                .times(1)
                .with(eq(RegisterOutputsRequest::new(HashMap::from([(
                    "output".into(),
                    NodeValue::exists(1, false),
                )]))))
                .returning(|_| ());

            let mut engine = Engine::new_without_configs(mock);

            let output_id = engine.create_done_node(1.into(), false);
            engine.add_output("output".into(), output_id);

            let _ = engine.run().await;
            let _ = engine.run().await;
        }
    }

    mod mapping {
        use super::*;
        use pulumi_gestalt_domain::connector::MockPulumiConnector;

        #[tokio::test]
        async fn should_return_map_function() {
            use serde_json::json;

            let mock = MockPulumiConnector::new();
            let mut engine = Engine::new_without_configs(mock);

            let source_output = engine.create_done_node(json!("value"), false);
            let mapped_output =
                engine.create_native_function_node("nativeFunc".into(), source_output);
            _ = engine.add_output("mapped_output".into(), mapped_output);

            let result = engine.run().await;

            match result {
                None => {
                    panic!("Expected NativeFunctionRequest, got None");
                }
                Some(request) => {
                    assert_eq!(request.function_name, "nativeFunc".into());
                    assert_eq!(request.data, json!("value"));
                }
            }
        }

        #[tokio::test]
        async fn should_invoke_map_even_if_it_is_not_in_output() {
            use serde_json::json;

            let mut mock = MockPulumiConnector::new();
            mock.expect_register_outputs()
                .times(1)
                .with(eq(RegisterOutputsRequest::new(HashMap::new())))
                .returning(|_| ());
            let mut engine = Engine::new_without_configs(mock);

            let source_output = engine.create_done_node(json!("value"), false);
            let _ = engine.create_native_function_node("nativeFunc".into(), source_output);

            let result = engine.run().await;

            match result {
                None => {
                    panic!("Expected NativeFunctionRequest, got None");
                }
                Some(request) => {
                    assert_eq!(request.function_name, "nativeFunc".into());
                    assert_eq!(request.data, json!("value"));
                }
            }
        }

        #[tokio::test]
        async fn should_handle_function_result() {
            use crate::Engine;

            let mut mock = MockPulumiConnector::new();
            mock.expect_register_outputs()
                .times(1)
                .with(eq(RegisterOutputsRequest::new(HashMap::from([(
                    "mapped_output".into(),
                    NodeValue::exists("result", false),
                )]))))
                .returning(|_| ());
            let mut engine = Engine::new_without_configs(mock);

            let source_output = engine.create_done_node("value".into(), false);
            let mapped_output =
                engine.create_native_function_node("nativeFunc".into(), source_output);
            engine.add_output("mapped_output".into(), mapped_output);

            let result = engine.run().await;

            match result {
                None => {
                    panic!("Expected NativeFunctionRequest, got None");
                }
                Some(request) => {
                    request.return_mailbox.send("result".into()).unwrap();
                }
            }

            let result = engine.run().await;

            match result {
                None => {}
                Some(_) => {
                    panic!("Expected None, got NativeFunctionRequest");
                }
            }
        }
    }

    mod create_combine_outputs {
        use super::*;
        use pulumi_gestalt_domain::connector::MockPulumiConnector;
        use serde_json::json;

        #[tokio::test]
        async fn should_combine_outputs() {
            use serde_json::json;

            let mut mock = MockPulumiConnector::new();

            let mut engine = Engine::new_without_configs(mock);

            let output1 = engine.create_done_node(json!("1"), false);
            let output2 = engine.create_done_node(json!(2), false);

            let combined_output = engine.create_combine_outputs(vec![output1, output2]);
            let result = combined_output.value.await;
            assert_eq!(result, NodeValue::exists(json!(["1", 2]), false));
        }

        #[tokio::test]
        async fn single_nothing_output_results_in_nothing() {
            let mut mock = MockPulumiConnector::new();

            let mut engine = Engine::new_without_configs(mock);

            let output1 = engine.create_nothing_node();
            let output2 = engine.create_done_node(json!(2), false);

            let combined_output = engine.create_combine_outputs(vec![output1, output2]);
            let result = combined_output.value.await;
            assert_eq!(result, NodeValue::Nothing);
        }

        #[tokio::test]
        async fn single_secret_output_is_secret() {
            use serde_json::json;

            let mut mock = MockPulumiConnector::new();

            let mut engine = Engine::new_without_configs(mock);

            let output1 = engine.create_done_node(json!("1"), false);
            let output2 = engine.create_done_node(json!(2), true);

            let combined_output = engine.create_combine_outputs(vec![output1, output2]);
            let result = combined_output.value.await;
            assert_eq!(result, NodeValue::exists(json!(["1", 2]), true));
        }
    }


    mod config {
        use super::*;
        use crate::config::Config;
        use crate::engine::ConfigValue;
        use pulumi_gestalt_domain::NodeValue;

        use std::collections::HashSet;
        use pulumi_gestalt_domain::connector::MockPulumiConnector;

        #[test]
        fn should_return_none_when_config_is_not_set() {
            let config = Config::new(HashMap::new(), HashSet::new());
            let mut engine = Engine::new(MockPulumiConnector::new(), config);
            let value = engine.get_config_value("name", "key");
            match value {
                None => {}
                Some(_) => {
                    panic!("Expected None, got Some");
                }
            }
        }

        #[test]
        fn should_return_value_when_config_is_plain_text() {
            let config = Config::new(
                HashMap::from([("name:key".to_string(), "value".to_string())]),
                HashSet::new(),
            );
            let mut engine = Engine::new(MockPulumiConnector::new(), config);
            let value = engine.get_config_value("name", "key");
            match value {
                None => {
                    panic!("Expected Some, got None");
                }
                Some(ConfigValue::PlainText(text)) => {
                    assert_eq!(text, "value");
                }
                Some(_) => {
                    panic!("Expected PlainText, got Secret");
                }
            }
        }

        #[tokio::test]
        async fn should_return_secret_output_when_config_is_secret() {
            let config = Config::new(
                HashMap::from([("name:key".to_string(), "secret".to_string())]),
                HashSet::from(["name:key".to_string()]),
            );
            let mut engine = Engine::new(MockPulumiConnector::new(), config);
            let value = engine.get_config_value("name", "key");
            match value {
                None => {
                    panic!("Expected Some, got None");
                }
                Some(ConfigValue::Secret(output)) => {
                    let result = output.value.await;
                    match result {
                        NodeValue::Exists(ExistingNodeValue { value, secret }) => {
                            assert_eq!(value, Value::String("secret".to_string()));
                            assert!(secret);
                        }
                        _ => {
                            panic!("Expected Exists, got Nothing");
                        }
                    }
                }
                Some(_) => {
                    panic!("Expected Secret, got PlainText");
                }
            }
        }
    }
}
