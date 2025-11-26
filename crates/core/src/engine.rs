use crate::config::{Config, RawConfigValue};
use crate::{RawOutput, RegisterResourceOutput};
use futures::FutureExt;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded};
use futures::channel::oneshot::{Sender, channel};
use futures::future::{BoxFuture, Shared};
use futures::stream::StreamExt;
use futures::stream::{FuturesOrdered, FuturesUnordered};
use pulumi_gestalt_domain::connector::{
    PulumiConnector, RegisterOutputsRequest, RegisterResourceRequest, ResourceInvokeRequest,
};
use pulumi_gestalt_domain::{ExistingNodeValue, FieldName, NodeValue};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

struct InternalNativeFunctionRequest {
    pub context_key: Uuid,
    pub data: Value,
    pub return_mailbox: Sender<Value>,
}

pub struct NativeFunctionRequest<FunctionContext> {
    pub context: FunctionContext,
    pub data: Value,
    pub return_mailbox: Sender<Value>,
}

pub enum ConfigValue {
    PlainText(String),
    Secret(RawOutput),
}

pub struct Engine<FunctionContext> {
    outputs: Mutex<HashMap<FieldName, RawOutput>>,
    function_contexts: Mutex<HashMap<Uuid, FunctionContext>>,
    join_set: FuturesUnordered<Shared<BoxFuture<'static, ()>>>,
    native_function_queue_sender: UnboundedSender<InternalNativeFunctionRequest>,
    native_function_queue_receiver: UnboundedReceiver<InternalNativeFunctionRequest>,

    output_task_created: AtomicBool,
    pulumi: Arc<Box<dyn PulumiConnector>>,
    config: Config,
}

type UnitEngine = Engine<()>;

impl<FunctionContext> Engine<FunctionContext> {
    pub fn new(pulumi: impl PulumiConnector + 'static, config: Config) -> Self {
        let (tx, rx) = unbounded();
        Self {
            outputs: Default::default(),
            function_contexts: Default::default(),
            join_set: Default::default(),
            pulumi: Arc::new(Box::new(pulumi)),
            output_task_created: AtomicBool::new(false),
            native_function_queue_sender: tx,
            native_function_queue_receiver: rx,
            config,
        }
    }

    #[cfg(test)]
    pub fn new_without_configs(pulumi: impl PulumiConnector + 'static) -> Self {
        use std::collections::HashSet;
        let config = Config::new(HashMap::new(), HashSet::new(), "project".to_string());
        Self::new(pulumi, config)
    }

    pub fn add_output(&self, field_name: FieldName, output_id: RawOutput) {
        let mut outputs = self.outputs.lock().unwrap();
        outputs.insert(field_name, output_id);
    }

    pub fn create_register_resource_node(
        &self,
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
        self.join_set.push(output.clone().invoke_void());

        output
    }

    pub fn create_resource_invoke_node(
        &self,
        token: String,
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
        self.join_set.push(output.clone().invoke_void());

        output
    }

    pub fn create_native_function_node(
        &self,
        function_context: FunctionContext,
        source: RawOutput,
    ) -> RawOutput {
        let function_context_key = Uuid::now_v7();
        let mut function_contexts = self.function_contexts.lock().unwrap();
        function_contexts.insert(function_context_key, function_context);
        drop(function_contexts);
        let request_receiver = self.native_function_queue_sender.clone();
        let output = RawOutput::from_future(async move {
            let source_value = source.value.await;
            match source_value {
                NodeValue::Nothing => NodeValue::Nothing,
                NodeValue::Exists(ExistingNodeValue { value, secret }) => {
                    let (tx, rx) = channel();
                    let request = InternalNativeFunctionRequest {
                        context_key: function_context_key,
                        data: value,
                        return_mailbox: tx,
                    };
                    request_receiver.unbounded_send(request).unwrap();

                    let result = rx.await.unwrap();
                    NodeValue::exists(result, secret)
                }
            }
        });
        self.join_set.push(output.clone().invoke_void());
        output
    }

    pub fn create_combine_outputs(&self, outputs: Vec<RawOutput>) -> RawOutput {
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

    pub fn create_done_node(value: Value, secret: bool) -> RawOutput {
        let node_value = NodeValue::exists(value, secret);
        RawOutput::from_node_value(node_value)
    }

    pub fn create_extract_field(
        field_name: FieldName,
        source: RegisterResourceOutput,
    ) -> RawOutput {
        RawOutput::from_future(async move {
            let resource_fields = source.value.await;
            resource_fields.get_field_value(&field_name)
        })
    }

    #[cfg(test)]
    fn create_nothing_node() -> RawOutput {
        RawOutput::from_node_value(NodeValue::Nothing)
    }

    pub async fn run(&mut self) -> Option<NativeFunctionRequest<FunctionContext>> {
        if self
            .output_task_created
            .compare_exchange(false, true, SeqCst, SeqCst)
            .is_ok()
        {
            let outputs = self.outputs.lock().unwrap();
            let outputs_map = outputs.clone();
            drop(outputs);
            let pulumi = self.pulumi.clone();

            let f = async move {
                let mut resolved_outputs = HashMap::new();
                for (key, output) in outputs_map {
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

            self.join_set.push(f.boxed().shared());
        }

        loop {
            futures::select! {
                res = self.join_set.next() => {
                    match res {
                        Some(_) => {
                            continue
                        }
                        None => {
                            // All tasks complete
                            // Tokio and futures structs have different behaviors when streams complete
                            // Tokio will be returning None, but futures will not return anything anymore
                            // Due to that if someone calls run again the select will wait for receiver - and since
                            // there is no more tasks it will be stuck forever. To avoid that we close the receiver here.
                            // That way futures will panic
                            self.native_function_queue_receiver.close();
                            return None;
                        }
                    }
                }
                request = self.native_function_queue_receiver.next() => {
                    match request {
                        Some(request) => {
                            let mut function_contexts = self.function_contexts.lock().unwrap();
                            let function_context = function_contexts.remove(&request.context_key).unwrap();
                            return Some(NativeFunctionRequest {
                                context: function_context,
                                data: request.data,
                                return_mailbox: request.return_mailbox,
                            });
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }
        }
    }

    pub fn get_config_value(&self, name: Option<&str>, key: &str) -> Option<ConfigValue> {
        match self.config.get(name, key) {
            None => None,
            Some(RawConfigValue::PlainText(value)) => Some(ConfigValue::PlainText(value.clone())),
            Some(RawConfigValue::Secret(secret)) => {
                let value = Value::String(secret.clone());
                let output_id = UnitEngine::create_done_node(value, true);
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
    use std::fmt::Debug;
    use std::sync::RwLock;

    static_assertions::assert_impl_all!(Engine<RwLock<()>>: Send, Sync);

    type StrEngine = Engine<&'static str>;

    mod register_outputs {
        use super::*;
        use pulumi_gestalt_domain::connector::MockPulumiConnector;

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

            let mut engine = StrEngine::new_without_configs(mock);

            let output_id = StrEngine::create_done_node(1.into(), false);
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

            let mut engine = StrEngine::new_without_configs(mock);

            let output_id = StrEngine::create_done_node(1.into(), false);
            engine.add_output("output".into(), output_id.clone());
            engine.create_native_function_node("nativeFunc".into(), output_id.clone());
            engine.create_native_function_node("nativeFunc2".into(), output_id);

            let result = engine.run().await;
            // nativeFunc
            assert!(result.is_some());

            let result = engine.run().await;
            // nativeFunc2
            assert!(result.is_some());
        }
    }

    mod mapping {
        use super::*;
        use pulumi_gestalt_domain::connector::MockPulumiConnector;

        #[tokio::test]
        async fn should_return_map_function() {
            use serde_json::json;

            let mock = MockPulumiConnector::new();
            let mut engine = StrEngine::new_without_configs(mock);

            let source_output = StrEngine::create_done_node(json!("value"), false);
            let mapped_output =
                engine.create_native_function_node("nativeFunc".into(), source_output);
            _ = engine.add_output("mapped_output".into(), mapped_output);

            let result = engine.run().await;

            match result {
                None => {
                    panic!("Expected NativeFunctionRequest, got None");
                }
                Some(request) => {
                    assert_eq!(request.context, "nativeFunc");
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
            let mut engine = StrEngine::new_without_configs(mock);

            let source_output = StrEngine::create_done_node(json!("value"), false);
            let _ = engine.create_native_function_node("nativeFunc".into(), source_output);

            let result = engine.run().await;

            match result {
                None => {
                    panic!("Expected NativeFunctionRequest, got None");
                }
                Some(request) => {
                    assert_eq!(request.context, "nativeFunc");
                    assert_eq!(request.data, json!("value"));
                }
            }
        }

        #[tokio::test]
        async fn should_handle_function_result() {
            let mut mock = MockPulumiConnector::new();
            mock.expect_register_outputs()
                .times(1)
                .with(eq(RegisterOutputsRequest::new(HashMap::from([(
                    "mapped_output".into(),
                    NodeValue::exists("result", false),
                )]))))
                .returning(|_| ());
            let mut engine = StrEngine::new_without_configs(mock);

            let source_output = StrEngine::create_done_node("value".into(), false);
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

            let mut engine = StrEngine::new_without_configs(mock);

            let output1 = StrEngine::create_done_node(json!("1"), false);
            let output2 = StrEngine::create_done_node(json!(2), false);

            let combined_output = engine.create_combine_outputs(vec![output1, output2]);
            let result = combined_output.value.await;
            assert_eq!(result, NodeValue::exists(json!(["1", 2]), false));
        }

        #[tokio::test]
        async fn single_nothing_output_results_in_nothing() {
            let mut mock = MockPulumiConnector::new();

            let mut engine = StrEngine::new_without_configs(mock);

            let output1 = StrEngine::create_nothing_node();
            let output2 = StrEngine::create_done_node(json!(2), false);

            let combined_output = engine.create_combine_outputs(vec![output1, output2]);
            let result = combined_output.value.await;
            assert_eq!(result, NodeValue::Nothing);
        }

        #[tokio::test]
        async fn single_secret_output_is_secret() {
            use serde_json::json;

            let mut mock = MockPulumiConnector::new();

            let mut engine = StrEngine::new_without_configs(mock);

            let output1 = StrEngine::create_done_node(json!("1"), false);
            let output2 = StrEngine::create_done_node(json!(2), true);

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

        use pulumi_gestalt_domain::connector::MockPulumiConnector;
        use std::collections::HashSet;

        #[test]
        fn should_return_none_when_config_is_not_set() {
            let config = Config::new(HashMap::new(), HashSet::new(), "project".to_string());
            let mut engine = StrEngine::new(MockPulumiConnector::new(), config);
            let value = engine.get_config_value(Some("name"), "key");
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
                "project".to_string(),
            );
            let mut engine = StrEngine::new(MockPulumiConnector::new(), config);
            let value = engine.get_config_value(Some("name"), "key");
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

        #[test]
        fn passing_none_will_use_project_name() {
            let config = Config::new(
                HashMap::from([("project:key".to_string(), "value".to_string())]),
                HashSet::new(),
                "project".to_string(),
            );
            let mut engine = StrEngine::new(MockPulumiConnector::new(), config);
            let value = engine.get_config_value(None, "key");
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
                "project".to_string(),
            );
            let mut engine = StrEngine::new(MockPulumiConnector::new(), config);
            let value = engine.get_config_value(Some("name"), "key");
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
