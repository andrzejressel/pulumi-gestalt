use anyhow::Context as AnyhowContext;
use futures::lock::Mutex;
use pulumi_gestalt_core as core;
use pulumi_gestalt_core::{Config, Engine};
use pulumi_gestalt_domain::{FieldName, NodeValue};
use pulumi_gestalt_grpc_connection::RealPulumiConnector;
use pulumi_gestalt_model::{PulumiValue, PulumiValueContent};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::future::Future;
use std::sync::Arc;

#[derive(bon::Builder)]
pub struct Context<FunctionContext> {
    inner: Arc<Mutex<core::Engine<FunctionContext>>>,
    stack: String,
    project: String,
    organization: String,
    root_directory: String,
}

pub struct Output<FunctionContext> {
    inner: core::RawOutput,
    engine: Arc<Mutex<core::Engine<FunctionContext>>>,
}

impl<T> Clone for Output<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            engine: Arc::clone(&self.engine),
        }
    }
}

pub struct RegisterResourceOutput<FunctionContext> {
    pub(crate) inner: core::RegisterResourceOutput,
    pub(crate) engine: Arc<Mutex<core::Engine<FunctionContext>>>,
}

pub struct NativeFunctionRequest<FunctionContext> {
    pub context: FunctionContext,
    pub data: PulumiValue,
    pub(crate) return_mailbox: futures::channel::oneshot::Sender<Value>,
}

pub struct RegisterResourceRequest<FunctionContext> {
    pub r#type: String,
    pub name: String,
    pub inputs: HashMap<FieldName, Output<FunctionContext>>,
    pub version: String,
    pub provider: Option<Output<FunctionContext>>,
}

pub struct InvokeResourceRequest<FunctionContext> {
    pub token: String,
    pub inputs: HashMap<FieldName, Output<FunctionContext>>,
    pub version: String,
}

pub enum ConfigValue<FunctionContext> {
    PlainText(String),
    Secret(Output<FunctionContext>),
}

impl<T> Context<T> {
    pub async fn new() -> Context<T> {
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

        Context::<T>::builder()
            .inner(Arc::new(Mutex::new(Engine::new(pulumi_connector, config))))
            .stack(pulumi_stack)
            .project(pulumi_project)
            .organization(pulumi_organization)
            .root_directory(pulumi_root_directory)
            .build()
    }

    pub async fn add_output(&self, field_name: FieldName, output: Output<T>) {
        self.inner.lock().await.add_output(field_name, output.inner)
    }

    pub async fn register_resource(
        &self,
        args: RegisterResourceRequest<T>,
    ) -> RegisterResourceOutput<T> {
        let inputs = args.inputs.into_iter().map(|(k, v)| (k, v.inner)).collect();
        let provider = args.provider.map(|p| p.inner);
        let inner = self.inner.lock().await.create_register_resource_node(
            args.r#type,
            args.name,
            inputs,
            args.version,
            provider,
        );
        RegisterResourceOutput {
            inner,
            engine: Arc::clone(&self.inner),
        }
    }

    pub async fn invoke_resource(
        &self,
        args: InvokeResourceRequest<T>,
    ) -> RegisterResourceOutput<T> {
        let inputs = args.inputs.into_iter().map(|(k, v)| (k, v.inner)).collect();
        let inner =
            self.inner
                .lock()
                .await
                .create_resource_invoke_node(args.token, inputs, args.version);
        RegisterResourceOutput {
            inner,
            engine: Arc::clone(&self.inner),
        }
    }

    pub async fn create_native_function_node(
        &self,
        function_context: T,
        source: Output<T>,
    ) -> Output<T> {
        let raw_output = self
            .inner
            .lock()
            .await
            .create_native_function_node(function_context, source.inner);
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.inner),
        }
    }

    pub async fn create_combine_outputs(&self, outputs: Vec<Output<T>>) -> Output<T> {
        let raw_outputs: Vec<core::RawOutput> = outputs.into_iter().map(|o| o.inner).collect();
        let raw_output = self.inner.lock().await.create_combine_outputs(raw_outputs);
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.inner),
        }
    }

    pub fn create_output(&self, value: PulumiValue) -> Output<T> {
        let raw_output =
            core::RawOutput::from_future_node_value(
                async move { pulumi_value_to_node_value(value) },
            );
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.inner),
        }
    }

    pub fn create_output_from_future<F>(&self, future: F) -> Output<T>
    where
        F: Future<Output = PulumiValue> + Send + 'static,
    {
        let raw_output = core::RawOutput::from_future_node_value(async move {
            pulumi_value_to_node_value(future.await)
        });
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.inner),
        }
    }

    pub async fn get_config_value(&self, name: Option<&str>, key: &str) -> Option<ConfigValue<T>> {
        self.inner
            .lock()
            .await
            .get_config_value(name, key)
            .map(|v| match v {
                core::ConfigValue::PlainText(s) => ConfigValue::PlainText(s),
                core::ConfigValue::Secret(o) => ConfigValue::Secret(Output {
                    inner: o,
                    engine: Arc::clone(&self.inner),
                }),
            })
    }

    pub async fn finish(&self) -> Option<NativeFunctionRequest<T>> {
        self.inner
            .lock()
            .await
            .run()
            .await
            .map(|request| NativeFunctionRequest {
                context: request.context,
                data: json_value_to_pulumi_value(request.data, false),
                return_mailbox: request.return_mailbox,
            })
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

impl<T> Output<T> {
    pub fn secret(&self) -> Self {
        Output {
            inner: self.inner.secret(),
            engine: Arc::clone(&self.engine),
        }
    }

    pub fn unsecret(&self) -> Self {
        Output {
            inner: self.inner.unsecret(),
            engine: Arc::clone(&self.engine),
        }
    }

    pub async fn map(&self, func: T) -> Self {
        let raw_output = self
            .engine
            .lock()
            .await
            .create_native_function_node(func, self.inner.clone());
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.engine),
        }
    }

    pub async fn combine(&self, others: &[&Output<T>]) -> Self {
        let mut all_outputs = vec![self.inner.clone()];
        for other in others {
            all_outputs.push(other.inner.clone());
        }
        let raw_output = self.engine.lock().await.create_combine_outputs(all_outputs);
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.engine),
        }
    }

    pub async fn add_export(&self, key: FieldName) {
        self.engine.lock().await.add_output(key, self.inner.clone());
    }

    pub async fn resolve_pulumi_value(&self) -> PulumiValue {
        node_value_to_pulumi_value(self.inner.resolve_node_value().await)
    }
}

impl<T> RegisterResourceOutput<T> {
    pub async fn get_field(&self, field_name: FieldName) -> Output<T> {
        let raw_output = core::Engine::<T>::create_extract_field(field_name, self.inner.clone());
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.engine),
        }
    }

    pub async fn get_urn(&self) -> Output<T> {
        let raw_output = core::Engine::<T>::create_extract_urn(self.inner.clone());
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.engine),
        }
    }

    pub async fn get_id(&self) -> Output<T> {
        let raw_output = core::Engine::<T>::create_extract_id(self.inner.clone());
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.engine),
        }
    }

    pub async fn get_provider_id(&self) -> Output<T> {
        let raw_output = core::Engine::<T>::create_extract_provider_id(self.inner.clone());
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.engine),
        }
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
