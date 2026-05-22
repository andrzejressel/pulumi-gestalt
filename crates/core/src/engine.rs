use crate::config::{Config, RawConfigValue};
use crate::{Output, RawOutput, RegisterResourceOutput};
use futures::FutureExt;
use futures::future::{BoxFuture, Shared};
use futures::stream::StreamExt;
use futures::stream::{FuturesOrdered, FuturesUnordered};
use pulumi_gestalt_domain::FieldName;
use pulumi_gestalt_domain::connector::{
    PulumiConnector, RegisterOutputsRequest, RegisterResourceRequest, ResourceInvokeRequest,
};
use pulumi_gestalt_model::{PulumiValue, PulumiValueContent};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, Mutex};

pub enum ConfigValue {
    PlainText(String),
    Secret(RawOutput),
}

pub struct Engine {
    outputs: Mutex<HashMap<FieldName, RawOutput>>,
    join_set: FuturesUnordered<Shared<BoxFuture<'static, ()>>>,
    output_task_created: AtomicBool,
    pulumi: Arc<Box<dyn PulumiConnector>>,
    config: Config,
}

impl Engine {
    pub fn new(pulumi: impl PulumiConnector + 'static, config: Config) -> Self {
        Self {
            outputs: Default::default(),
            join_set: Default::default(),
            pulumi: Arc::new(Box::new(pulumi)),
            output_task_created: AtomicBool::new(false),
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
        provider: Option<RawOutput>,
    ) -> RegisterResourceOutput {
        let pulumi = self.pulumi.clone();
        let result = Output::from_future(async move {
            let mut resolved_inputs = HashMap::new();
            for (key, output) in inputs {
                let value = output.value.await;
                resolved_inputs.insert(key, value);
            }

            let provider_id = match provider {
                None => None,
                Some(p) => match p.value.await {
                    PulumiValue {
                        content: PulumiValueContent::String(s),
                        ..
                    } => Some(s),
                    PulumiValue {
                        content: PulumiValueContent::Nothing,
                        ..
                    } => None,
                    v => panic!("Expected Provider URN to be a String, got {:?}", v.content),
                },
            };

            let result = pulumi
                .register_resource(
                    RegisterResourceRequest::builder()
                        .r#type(r#type)
                        .name(name)
                        .version(version)
                        .object(resolved_inputs)
                        .maybe_provider(provider_id)
                        .build(),
                )
                .await;

            (Arc::new(result.fields), result.urn, result.id)
        });
        let fields = Output::from_future({
            let result = result.clone();
            async move {
                let (fields, _, _) = result.value.await;
                fields
            }
        });
        let urn = RawOutput::from_future({
            let result = result.clone();
            async move {
                let (_, urn, _) = result.value.await;
                urn
            }
        });
        let id = RawOutput::from_future(async move {
            let (_, _, id) = result.value.await;
            id
        });
        let provider_id = RawOutput::from_future({
            let urn = urn.clone();
            let id = id.clone();
            async move {
                let urn_val = urn.value.await;
                let id_val = id.value.await;
                match (urn_val, id_val) {
                    (
                        PulumiValue {
                            content: PulumiValueContent::String(urn_str),
                            secret: urn_secret,
                            ..
                        },
                        PulumiValue {
                            content: PulumiValueContent::String(id_str),
                            secret: id_secret,
                            ..
                        },
                    ) => PulumiValue {
                        content: PulumiValueContent::String(format!("{}::{}", urn_str, id_str)),
                        secret: urn_secret || id_secret,
                        dependencies: Default::default(),
                    },
                    _ => PulumiValue::nothing(),
                }
            }
        });

        let output = RegisterResourceOutput {
            fields,
            urn,
            id,
            provider_id,
        };
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
        let fields = Output::from_future(async move {
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
        let urn = RawOutput::from_pulumi_value(PulumiValue::nothing());
        let id = RawOutput::from_pulumi_value(PulumiValue::nothing());
        let provider_id = RawOutput::from_pulumi_value(PulumiValue::nothing());
        let output = RegisterResourceOutput {
            fields,
            urn,
            id,
            provider_id,
        };
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
            let secret = results.iter().any(|res| res.secret);
            let mut dependencies = std::collections::HashSet::new();
            for res in &results {
                dependencies.extend(res.dependencies.iter().cloned());
            }

            for result in results {
                match result.content {
                    PulumiValueContent::Nothing => {
                        return PulumiValue::nothing();
                    }
                    _ => {
                        combined.push(result.to_json());
                    }
                }
            }

            PulumiValue {
                content: PulumiValueContent::Array(
                    combined
                        .into_iter()
                        .map(|v| PulumiValue::from_json(v, false))
                        .collect(),
                ),
                secret,
                dependencies,
            }
        })
    }

    pub fn create_done_node(value: Value, secret: bool) -> RawOutput {
        let pulumi_value = PulumiValue::from_json(value, secret);
        RawOutput::from_pulumi_value(pulumi_value)
    }

    pub fn create_extract_field(
        field_name: FieldName,
        source: RegisterResourceOutput,
    ) -> RawOutput {
        RawOutput::from_future(async move {
            let resource_fields = source.fields.value.await;
            resource_fields.get_field_value(&field_name)
        })
    }

    pub fn create_extract_urn(source: RegisterResourceOutput) -> RawOutput {
        source.get_urn()
    }

    pub fn create_extract_id(source: RegisterResourceOutput) -> RawOutput {
        source.get_id()
    }

    pub fn create_extract_provider_id(source: RegisterResourceOutput) -> RawOutput {
        source.get_provider_id()
    }

    #[cfg(test)]
    fn create_nothing_node() -> RawOutput {
        RawOutput::from_pulumi_value(PulumiValue::nothing())
    }

    pub async fn run(&mut self) {
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

        while self.join_set.next().await.is_some() {}
    }

    pub fn get_config_value(&self, name: Option<&str>, key: &str) -> Option<ConfigValue> {
        match self.config.get(name, key) {
            None => None,
            Some(RawConfigValue::PlainText(value)) => Some(ConfigValue::PlainText(value.clone())),
            Some(RawConfigValue::Secret(secret)) => {
                let value = Value::String(secret.clone());
                let output_id = Engine::create_done_node(value, true);
                Some(ConfigValue::Secret(output_id))
            }
        }
    }

    pub async fn require_pulumi_version(&self, version_range: &str) -> anyhow::Result<()> {
        self.pulumi.require_pulumi_version(version_range).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use std::collections::HashMap;

    static_assertions::assert_impl_all!(Engine: Send, Sync);

    type StrEngine = Engine;

    mod register_outputs {
        use super::*;
        use pulumi_gestalt_domain::connector::MockPulumiConnector;
        use pulumi_gestalt_model::{PulumiValue, PulumiValueContent};

        #[tokio::test]
        async fn should_register_outputs() {
            let mut mock = MockPulumiConnector::new();
            mock.expect_register_outputs()
                .times(1)
                .with(eq(RegisterOutputsRequest::new(HashMap::from([(
                    "output".into(),
                    PulumiValue {
                        content: PulumiValueContent::Integer(1),
                        secret: false,
                        dependencies: Default::default(),
                    },
                )]))))
                .returning(|_| ());

            let mut engine = StrEngine::new_without_configs(mock);

            let output_id = StrEngine::create_done_node(1.into(), false);
            engine.add_output("output".into(), output_id);

            engine.run().await;
        }

        #[tokio::test]
        async fn should_only_create_output_task_once() {
            let mut mock = MockPulumiConnector::new();
            mock.expect_register_outputs()
                .times(1)
                .with(eq(RegisterOutputsRequest::new(HashMap::from([(
                    "output".into(),
                    PulumiValue {
                        content: PulumiValueContent::Integer(1),
                        secret: false,
                        dependencies: Default::default(),
                    },
                )]))))
                .returning(|_| ());

            let mut engine = StrEngine::new_without_configs(mock);

            let output_id = StrEngine::create_done_node(1.into(), false);
            engine.add_output("output".into(), output_id);

            engine.run().await;
            engine.run().await;
        }
    }

    mod create_combine_outputs {
        use super::*;
        use pulumi_gestalt_domain::connector::MockPulumiConnector;
        use pulumi_gestalt_model::PulumiValueContent;
        use serde_json::json;

        #[tokio::test]
        async fn should_combine_outputs() {
            use serde_json::json;

            let mock = MockPulumiConnector::new();

            let engine = StrEngine::new_without_configs(mock);

            let output1 = StrEngine::create_done_node(json!("1"), false);
            let output2 = StrEngine::create_done_node(json!(2), false);

            let combined_output = engine.create_combine_outputs(vec![output1, output2]);
            let result = combined_output.value.await;
            assert_eq!(result.to_json(), json!(["1", 2]));
            assert!(!result.secret);
        }

        #[tokio::test]
        async fn single_nothing_output_results_in_nothing() {
            let mock = MockPulumiConnector::new();

            let engine = StrEngine::new_without_configs(mock);

            let output1 = StrEngine::create_nothing_node();
            let output2 = StrEngine::create_done_node(json!(2), false);

            let combined_output = engine.create_combine_outputs(vec![output1, output2]);
            let result = combined_output.value.await;
            assert!(matches!(result.content, PulumiValueContent::Nothing));
        }

        #[tokio::test]
        async fn single_secret_output_is_secret() {
            use serde_json::json;

            let mock = MockPulumiConnector::new();

            let engine = StrEngine::new_without_configs(mock);

            let output1 = StrEngine::create_done_node(json!("1"), false);
            let output2 = StrEngine::create_done_node(json!(2), true);

            let combined_output = engine.create_combine_outputs(vec![output1, output2]);
            let result = combined_output.value.await;
            assert_eq!(result.to_json(), json!(["1", 2]));
            assert!(result.secret);
        }
    }

    mod config {
        use super::*;
        use crate::config::Config;
        use crate::engine::ConfigValue;
        use pulumi_gestalt_model::PulumiValueContent;

        use pulumi_gestalt_domain::connector::MockPulumiConnector;
        use std::collections::HashSet;

        #[test]
        fn should_return_none_when_config_is_not_set() {
            let config = Config::new(HashMap::new(), HashSet::new(), "project".to_string());
            let engine = StrEngine::new(MockPulumiConnector::new(), config);
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
            let engine = StrEngine::new(MockPulumiConnector::new(), config);
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
            let engine = StrEngine::new(MockPulumiConnector::new(), config);
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
            let engine = StrEngine::new(MockPulumiConnector::new(), config);
            let value = engine.get_config_value(Some("name"), "key");
            match value {
                None => {
                    panic!("Expected Some, got None");
                }
                Some(ConfigValue::Secret(output)) => {
                    let result = output.value.await;
                    assert_eq!(result.to_json(), Value::String("secret".to_string()));
                    assert!(matches!(result.content, PulumiValueContent::String(_)));
                    assert!(result.secret);
                }
                Some(_) => {
                    panic!("Expected Secret, got PlainText");
                }
            }
        }
    }
}
