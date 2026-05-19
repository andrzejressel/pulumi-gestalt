use crate::PulumiAny;
use anyhow::{Context as anyhowContext, Result, bail};
use bon::Builder;
use pulumi_gestalt_model::Output;
use pulumi_gestalt_model::any_export::IntoOutputAny;
use pulumi_gestalt_rust_integration as integration;
use pulumi_gestalt_rust_integration::{ConfigValue, FieldName, NodeValue};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;

pub trait Provider {
    fn get_provider_id(&self) -> Output<String>;
}

#[derive(Default, Builder, Clone)]
pub struct CustomResourceOptions {
    #[builder(with = |p: &impl Provider| { p.get_provider_id() })]
    pub provider: Option<Output<String>>,
}

pub type FunctionContext = Box<dyn Fn(Value) -> Value + Send>;

pub struct CompositeOutput {
    inner: integration::RegisterResourceOutput<FunctionContext>,
    runtime: Arc<Runtime>,
}

impl CompositeOutput {
    fn from_internal(
        inner: integration::RegisterResourceOutput<FunctionContext>,
        runtime: Arc<Runtime>,
    ) -> Self {
        Self { inner, runtime }
    }

    pub fn get_field<T>(&self, key: &str) -> Output<T>
    where
        T: DeserializeOwned + Send + Sync + 'static,
    {
        let res = self
            .runtime
            .block_on(self.inner.get_field(FieldName::from(key.to_string())));
        Context::integration_to_model_output(res)
    }

    pub fn get_urn(&self) -> Output<String> {
        let res = self.runtime.block_on(self.inner.get_urn());
        Context::integration_to_model_output(res)
    }

    pub fn get_id(&self) -> Output<String> {
        let res = self.runtime.block_on(self.inner.get_id());
        Context::integration_to_model_output(res)
    }

    pub fn get_provider_id(&self) -> Output<String> {
        let res = self.runtime.block_on(self.inner.get_provider_id());
        Context::integration_to_model_output(res)
    }
}

pub struct Context {
    inner: integration::Context<FunctionContext>,
    runtime: Arc<Runtime>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        let runtime = Runtime::new().unwrap();
        let ctx = runtime.block_on(integration::Context::new());
        Self {
            inner: ctx,
            runtime: Arc::new(runtime),
        }
    }

    pub fn finish(&self) {
        self.runtime.block_on(
            pulumi_gestalt_rust_integration::finish::finish_lambdas_sequentially(&self.inner),
        )
    }

    pub fn new_output<'a, T>(&self, value: &'a T) -> Output<T>
    where
        T: Serialize + Deserialize<'a> + Clone + Send + Sync + 'static,
    {
        let value = value.clone();
        Output::from_resolved_future(async move {
            pulumi_gestalt_model::ResolvedOutput {
                value: Some(value),
                secret: false,
                dependencies: Default::default(),
            }
        })
    }

    pub fn new_secret<T>(&self, value: &T) -> Output<T>
    where
        T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
    {
        self.new_output(value).secret()
    }

    pub fn add_export(&self, key: &str, output: &impl IntoOutputAny) {
        let internal_output =
            self.model_to_integration_output(output.as_output().map(PulumiAny::from));
        self.runtime
            .block_on(internal_output.add_export(FieldName::from(key.to_string())));
    }

    pub fn register_resource(&self, request: RegisterResourceRequest) -> CompositeOutput {
        let mut inputs = HashMap::new();
        for object in request.object {
            inputs.insert(
                FieldName::from(object.name.clone()),
                object.value.as_internal_output(self),
            );
        }
        let provider = request
            .options
            .as_ref()
            .and_then(|o| o.provider.as_ref())
            .map(|p| self.model_to_integration_output(p.clone()));

        let result = self.runtime.block_on(self.inner.register_resource(
            integration::RegisterResourceRequest {
                r#type: request.type_.clone(),
                name: request.name.clone(),
                version: request.version.clone(),
                inputs,
                provider,
            },
        ));

        CompositeOutput::from_internal(result, self.runtime.clone())
    }

    pub fn invoke_resource(&self, request: InvokeResourceRequest) -> CompositeOutput {
        let mut inputs = HashMap::new();
        for object in request.object {
            inputs.insert(
                FieldName::from(object.name.clone()),
                object.value.as_internal_output(self),
            );
        }
        let result = self.runtime.block_on(self.inner.invoke_resource(
            integration::InvokeResourceRequest {
                token: request.token.clone(),
                version: request.version.clone(),
                inputs,
            },
        ));
        CompositeOutput::from_internal(result, self.runtime.clone())
    }

    fn model_to_integration_output<T>(
        &self,
        output: Output<T>,
    ) -> integration::Output<FunctionContext>
    where
        T: Serialize + Clone + Send + Sync + 'static,
    {
        self.inner.create_output_from_future(async move {
            let resolved = output.resolve().await;
            match resolved.value {
                None => NodeValue::Nothing,
                Some(value) => {
                    NodeValue::exists(serde_json::to_value(value).unwrap(), resolved.secret)
                }
            }
        })
    }

    fn integration_to_model_output<T>(output: integration::Output<FunctionContext>) -> Output<T>
    where
        T: DeserializeOwned + Send + Sync + 'static,
    {
        Output::from_resolved_future(async move {
            match output.resolve_node_value().await {
                NodeValue::Nothing => pulumi_gestalt_model::ResolvedOutput {
                    value: None,
                    secret: false,
                    dependencies: Default::default(),
                },
                NodeValue::Exists(existing) => pulumi_gestalt_model::ResolvedOutput {
                    value: Some(serde_json::from_value(existing.value).unwrap()),
                    secret: existing.secret,
                    dependencies: Default::default(),
                },
            }
        })
    }

    fn config_full_key(name: Option<&str>, key: &str) -> String {
        let namespace = name
            .map(|value| value.to_string())
            .or_else(|| std::env::var("PULUMI_PROJECT").ok())
            .unwrap_or_else(|| "<unknown-project>".to_string());
        format!("{namespace}:{key}")
    }

    pub fn require_config(&self, name: Option<&str>, key: &str) -> Result<String> {
        let full_key = Self::config_full_key(name, key);
        match self
            .runtime
            .block_on(self.inner.get_config_value(name, key))
        {
            Some(ConfigValue::PlainText(value)) => Ok(value),
            Some(ConfigValue::Secret(_)) => {
                bail!("Config `{full_key}` is secret and cannot be read as plaintext")
            }
            None => {
                bail!("Config `{full_key}` does not exist")
            }
        }
    }

    pub fn require_config_deserialize<T>(&self, name: Option<&str>, key: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let val = self
            .require_config(name, key)
            .context("Failed to obtain config value")?;

        serde_json::from_str(&val)
            .with_context(|| format!("Failed to deserialize config value for key `{key}`"))
    }

    pub fn require_config_secret(&self, name: Option<&str>, key: &str) -> Result<Output<String>> {
        let full_key = Self::config_full_key(name, key);
        match self
            .runtime
            .block_on(self.inner.get_config_value(name, key))
        {
            Some(ConfigValue::Secret(sec)) => Ok(Self::integration_to_model_output(sec)),
            Some(ConfigValue::PlainText(_)) => {
                bail!("Config `{full_key}` is plaintext and cannot be read as secret")
            }
            None => {
                bail!("Config `{full_key}` does not exist")
            }
        }
    }

    pub fn require_config_secret_deserialize<T>(
        &self,
        name: Option<&str>,
        key: &str,
    ) -> Result<Output<T>>
    where
        T: for<'de> Deserialize<'de> + Serialize + Clone + Send + Sync + 'static,
    {
        let secret_output = self
            .require_config_secret(name, key)
            .context("Failed to obtain secret config value")?;

        let key = key.to_string();
        Ok(secret_output.map(move |s| {
            serde_json::from_str(&s)
                .with_context(|| {
                    format!("Failed to deserialize secret config value for key `{key}`")
                })
                .unwrap()
        }))
    }

    pub fn get_organization(&self) -> &str {
        self.inner.get_organization()
    }

    pub fn get_project(&self) -> &str {
        self.inner.get_project()
    }

    pub fn get_stack(&self) -> &str {
        self.inner.get_stack()
    }

    pub fn get_root_directory(&self) -> &str {
        self.inner.get_root_directory()
    }

    pub fn require_pulumi_version(&self, version_range: &str) -> Result<()> {
        self.runtime
            .block_on(self.inner.require_pulumi_version(version_range))
            .context("Failed to require Pulumi version")
    }
}

pub trait IntoInternalOutput {
    fn as_internal_output(&self, ctx: &Context) -> integration::Output<FunctionContext>;
}

impl<T> IntoInternalOutput for Output<T>
where
    T: Serialize + Clone + Send + Sync + 'static,
{
    fn as_internal_output(&self, ctx: &Context) -> integration::Output<FunctionContext> {
        ctx.model_to_integration_output(self.clone())
    }
}

pub struct RegisterResourceRequest<'a> {
    pub type_: String,
    pub name: String,
    pub version: String,
    pub object: &'a [ResourceRequestObjectField<'a>],
    pub options: Option<CustomResourceOptions>,
}

pub struct InvokeResourceRequest<'a> {
    pub token: String,
    pub version: String,
    pub object: &'a [ResourceRequestObjectField<'a>],
}

pub struct ResourceRequestObjectField<'a> {
    pub name: String,
    pub value: &'a dyn IntoInternalOutput,
}
