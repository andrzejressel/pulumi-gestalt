use anyhow::Context as AnyhowContext;
use futures::lock::Mutex;
use pulumi_gestalt_core as core;
use pulumi_gestalt_core::{Config, Engine};
use pulumi_gestalt_domain::{FieldName, NodeValue};
use pulumi_gestalt_grpc_connection::RealPulumiConnector;
use pulumi_gestalt_model::{PulumiValue, PulumiValueContent, ResolvedOutput};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::future::Future;
use std::sync::Arc;

#[derive(bon::Builder)]
pub struct Context {
    inner: Arc<Mutex<core::Engine>>,
    stack: String,
    project: String,
    organization: String,
    root_directory: String,
}

pub type Output = pulumi_gestalt_model::Output<PulumiValue>;

pub struct RegisterResourceOutput {
    pub(crate) inner: core::RegisterResourceOutput,
}

pub struct RegisterResourceRequest {
    pub r#type: String,
    pub name: String,
    pub inputs: HashMap<FieldName, Output>,
    pub version: String,
    pub provider: Option<Output>,
}

pub struct InvokeResourceRequest {
    pub token: String,
    pub inputs: HashMap<FieldName, Output>,
    pub version: String,
}

pub enum ConfigValue {
    PlainText(String),
    Secret(Output),
}

fn model_output_to_raw_output(output: Output) -> core::RawOutput {
    core::RawOutput::from_future_node_value(async move {
        let resolved = output.resolve().await;
        match resolved.value {
            Some(mut value) => {
                value.secret |= resolved.secret;
                value.dependencies.extend(resolved.dependencies);
                pulumi_value_to_node_value(value)
            }
            None => NodeValue::Nothing,
        }
    })
}

fn raw_output_to_model_output(output: core::RawOutput) -> Output {
    Output::from_resolved_future(async move {
        let pulumi_value = node_value_to_pulumi_value(output.resolve_node_value().await);
        if matches!(pulumi_value.content, PulumiValueContent::Nothing) {
            ResolvedOutput {
                value: None,
                secret: false,
                dependencies: HashSet::new(),
            }
        } else {
            ResolvedOutput {
                value: Some(pulumi_value.clone()),
                secret: pulumi_value.secret,
                dependencies: pulumi_value.dependencies,
            }
        }
    })
}

impl Context {
    pub async fn new() -> Context {
        let pulumi_engine_url = std::env::var("PULUMI_ENGINE").unwrap();
        let pulumi_monitor_url = std::env::var("PULUMI_MONITOR").unwrap();
        let pulumi_stack = std::env::var("PULUMI_STACK").unwrap();
        let pulumi_project = std::env::var("PULUMI_PROJECT").unwrap();
        let pulumi_organization = std::env::var("PULUMI_ORGANIZATION").unwrap();
        let pulumi_root_directory = std::env::var("PULUMI_ROOT_DIRECTORY").unwrap();
        let in_preview = match std::env::var("PULUMI_DRY_RUN") {
            Ok(preview) if preview == "true" => true,
            Ok(preview) if preview == "false" => false,
            _ => false,
        };

        let pulumi_connector = RealPulumiConnector::new(
            pulumi_monitor_url,
            pulumi_engine_url.clone(),
            pulumi_project.clone(),
            pulumi_stack.clone(),
            in_preview,
        )
        .await
        .context("Failed to create Pulumi connector")
        .unwrap();

        let config = Config::from_env_vars()
            .context("Failed to create config instance")
            .unwrap();

        Context::builder()
            .inner(Arc::new(Mutex::new(Engine::new(pulumi_connector, config))))
            .stack(pulumi_stack)
            .project(pulumi_project)
            .organization(pulumi_organization)
            .root_directory(pulumi_root_directory)
            .build()
    }

    pub async fn add_output(&self, field_name: FieldName, output: Output) {
        self.inner
            .lock()
            .await
            .add_output(field_name, model_output_to_raw_output(output))
    }

    pub async fn register_resource(&self, args: RegisterResourceRequest) -> RegisterResourceOutput {
        let inputs = args
            .inputs
            .into_iter()
            .map(|(k, v)| (k, model_output_to_raw_output(v)))
            .collect();
        let provider = args.provider.map(model_output_to_raw_output);
        let inner = self.inner.lock().await.create_register_resource_node(
            args.r#type,
            args.name,
            inputs,
            args.version,
            provider,
        );
        RegisterResourceOutput { inner }
    }

    pub async fn invoke_resource(&self, args: InvokeResourceRequest) -> RegisterResourceOutput {
        let inputs = args
            .inputs
            .into_iter()
            .map(|(k, v)| (k, model_output_to_raw_output(v)))
            .collect();
        let inner =
            self.inner
                .lock()
                .await
                .create_resource_invoke_node(args.token, inputs, args.version);
        RegisterResourceOutput { inner }
    }

    pub fn create_output(&self, value: PulumiValue) -> Output {
        Output::new(value)
    }

    pub fn create_output_from_future<F>(&self, future: F) -> Output
    where
        F: Future<Output = PulumiValue> + Send + 'static,
    {
        Output::from_resolved_future(async move {
            ResolvedOutput {
                value: Some(future.await),
                secret: false,
                dependencies: HashSet::new(),
            }
        })
    }

    pub async fn get_config_value(&self, name: Option<&str>, key: &str) -> Option<ConfigValue> {
        self.inner
            .lock()
            .await
            .get_config_value(name, key)
            .map(|v| match v {
                core::ConfigValue::PlainText(s) => ConfigValue::PlainText(s),
                core::ConfigValue::Secret(o) => ConfigValue::Secret(raw_output_to_model_output(o)),
            })
    }

    pub async fn finish(&self) {
        self.inner.lock().await.run().await
    }

    pub fn get_organization(&self) -> &str {
        &self.organization
    }

    pub fn get_project(&self) -> &str {
        &self.project
    }

    pub fn get_stack(&self) -> &str {
        &self.stack
    }

    pub fn get_root_directory(&self) -> &str {
        &self.root_directory
    }

    pub async fn require_pulumi_version(&self, version_range: &str) -> anyhow::Result<()> {
        self.inner
            .lock()
            .await
            .require_pulumi_version(version_range)
            .await
    }
}

impl RegisterResourceOutput {
    pub async fn get_field(&self, field_name: FieldName) -> Output {
        let raw_output = core::Engine::create_extract_field(field_name, self.inner.clone());
        raw_output_to_model_output(raw_output)
    }

    pub async fn get_urn(&self) -> Output {
        let raw_output = core::Engine::create_extract_urn(self.inner.clone());
        raw_output_to_model_output(raw_output)
    }

    pub async fn get_id(&self) -> Output {
        let raw_output = core::Engine::create_extract_id(self.inner.clone());
        raw_output_to_model_output(raw_output)
    }

    pub async fn get_provider_id(&self) -> Output {
        let raw_output = core::Engine::create_extract_provider_id(self.inner.clone());
        raw_output_to_model_output(raw_output)
    }
}

fn pulumi_value_to_node_value(value: PulumiValue) -> NodeValue {
    let PulumiValue {
        content,
        secret,
        dependencies: _,
    } = value;
    match content {
        PulumiValueContent::Nothing => NodeValue::Nothing,
        _ => NodeValue::exists(
            pulumi_value_to_json_value(PulumiValue {
                content,
                secret,
                dependencies: HashSet::new(),
            }),
            secret,
        ),
    }
}

fn node_value_to_pulumi_value(value: NodeValue) -> PulumiValue {
    match value {
        NodeValue::Nothing => PulumiValue {
            content: PulumiValueContent::Nothing,
            secret: false,
            dependencies: HashSet::new(),
        },
        NodeValue::Exists(existing) => json_value_to_pulumi_value(existing.value, existing.secret),
    }
}

fn json_value_to_pulumi_value(value: Value, secret: bool) -> PulumiValue {
    let content = match value {
        Value::Null => PulumiValueContent::None,
        Value::Bool(boolean) => PulumiValueContent::Boolean(boolean),
        Value::Number(number) => {
            if let Some(integer) = number.as_i64() {
                PulumiValueContent::Integer(
                    i32::try_from(integer)
                        .expect("i64 value is outside supported i32 range for Pulumi integers"),
                )
            } else {
                PulumiValueContent::Number(
                    number
                        .as_f64()
                        .expect("serde_json::Number must be convertible to f64"),
                )
            }
        }
        Value::String(string) => PulumiValueContent::String(string),
        Value::Array(values) => PulumiValueContent::Array(
            values
                .into_iter()
                .map(|v| json_value_to_pulumi_value(v, false))
                .collect(),
        ),
        Value::Object(values) => PulumiValueContent::Object(
            values
                .into_iter()
                .map(|(key, value)| (key, json_value_to_pulumi_value(value, false)))
                .collect(),
        ),
    };

    PulumiValue {
        content,
        secret,
        dependencies: HashSet::new(),
    }
}

pub(crate) fn pulumi_value_to_json_value(value: PulumiValue) -> Value {
    match value.content {
        PulumiValueContent::String(value) => Value::String(value),
        PulumiValueContent::Integer(value) => Value::from(value),
        PulumiValueContent::Number(value) => Value::from(value),
        PulumiValueContent::Boolean(value) => Value::from(value),
        PulumiValueContent::Array(values) => {
            Value::Array(values.into_iter().map(pulumi_value_to_json_value).collect())
        }
        PulumiValueContent::Object(values) => Value::Object(
            values
                .into_iter()
                .map(|(key, value)| (key, pulumi_value_to_json_value(value)))
                .collect(),
        ),
        PulumiValueContent::None | PulumiValueContent::Nothing => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        json_value_to_pulumi_value, node_value_to_pulumi_value, pulumi_value_to_json_value,
        pulumi_value_to_node_value,
    };
    use pulumi_gestalt_domain::NodeValue;
    use pulumi_gestalt_model::{PulumiValue, PulumiValueContent};
    use serde_json::json;
    use std::collections::HashSet;

    #[test]
    fn preserves_nothing_through_node_conversion() {
        let pulumi_value = PulumiValue {
            content: PulumiValueContent::Nothing,
            secret: true,
            dependencies: HashSet::new(),
        };

        let node_value = pulumi_value_to_node_value(pulumi_value);
        assert!(matches!(node_value, NodeValue::Nothing));

        let back = node_value_to_pulumi_value(node_value);
        assert!(matches!(back.content, PulumiValueContent::Nothing));
        assert!(!back.secret);
    }

    #[test]
    fn keeps_secret_when_mapping_existing_node() {
        let node_value = NodeValue::exists(json!("secret"), true);
        let pulumi_value = node_value_to_pulumi_value(node_value);

        assert_eq!(
            pulumi_value,
            PulumiValue {
                content: PulumiValueContent::String("secret".to_string()),
                secret: true,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn treats_json_null_as_none() {
        let pulumi_value = json_value_to_pulumi_value(serde_json::Value::Null, false);
        assert!(matches!(pulumi_value.content, PulumiValueContent::None));

        let json = pulumi_value_to_json_value(pulumi_value);
        assert_eq!(json, serde_json::Value::Null);
    }

    #[test]
    fn converts_nested_structures_round_trip() {
        let value = PulumiValue {
            content: PulumiValueContent::Object(vec![(
                "items".to_string(),
                PulumiValue {
                    content: PulumiValueContent::Array(vec![
                        PulumiValue {
                            content: PulumiValueContent::Integer(1),
                            secret: false,
                            dependencies: HashSet::new(),
                        },
                        PulumiValue {
                            content: PulumiValueContent::String("two".to_string()),
                            secret: false,
                            dependencies: HashSet::new(),
                        },
                    ]),
                    secret: false,
                    dependencies: HashSet::new(),
                },
            )]),
            secret: false,
            dependencies: HashSet::new(),
        };

        let json = pulumi_value_to_json_value(value.clone());
        let back = json_value_to_pulumi_value(json, false);
        assert_eq!(back, value);
    }
}
