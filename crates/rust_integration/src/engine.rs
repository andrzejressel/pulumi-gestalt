use anyhow::Context as AnyhowContext;
use futures::lock::Mutex;
use pulumi_gestalt_core as core;
use pulumi_gestalt_core::{Config, Engine};
use pulumi_gestalt_domain::FieldName;
use pulumi_gestalt_grpc_connection::RealPulumiConnector;
use pulumi_gestalt_model::{PulumiValue, PulumiValueContent, ResolvedOutput};
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
    core::RawOutput::from_future_pulumi_value(async move {
        let resolved = output.resolve().await;
        match resolved.value {
            Some(mut value) => {
                value.secret |= resolved.secret;
                value.dependencies.extend(resolved.dependencies);
                value
            }
            None => PulumiValue::nothing(),
        }
    })
}

fn raw_output_to_model_output(output: core::RawOutput) -> Output {
    Output::from_resolved_future(async move {
        let pulumi_value = output.resolve_pulumi_value().await;
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

#[cfg(test)]
mod tests {
    use super::{model_output_to_raw_output, raw_output_to_model_output};
    use crate::Output;
    use pulumi_gestalt_model::{PulumiValue, PulumiValueContent};
    use std::collections::HashSet;
    use tokio::runtime::Runtime;

    #[test]
    fn preserves_nothing_through_raw_output_conversion() {
        let pulumi_value = PulumiValue {
            content: PulumiValueContent::Nothing,
            secret: true,
            dependencies: HashSet::new(),
        };

        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let model_out = Output::new(pulumi_value);
            let raw = model_output_to_raw_output(model_out);
            let resolved = raw.resolve_pulumi_value().await;
            assert!(matches!(resolved.content, PulumiValueContent::Nothing));
        });
    }

    #[test]
    fn preserves_secret_and_dependencies_roundtrip() {
        let value = PulumiValue {
            content: PulumiValueContent::String("secret".to_string()),
            secret: true,
            dependencies: HashSet::from(["urn:1".to_string()]),
        };

        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let model = Output::new(value.clone());
            let raw = model_output_to_raw_output(model);
            let model_back = raw_output_to_model_output(raw);
            let resolved = model_back.resolve().await;
            assert_eq!(resolved.value, Some(value));
            assert!(resolved.secret);
            assert!(resolved.dependencies.contains("urn:1"));
        });
    }
}
