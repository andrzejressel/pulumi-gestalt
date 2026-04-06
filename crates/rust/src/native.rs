use anyhow::{Context as anyhowContext, Result, bail};
use bon::Builder;
use pulumi_gestalt_rust_integration as integration;
use pulumi_gestalt_rust_integration::{ConfigValue, FieldName};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, from_value, to_value};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use tokio::runtime::Runtime;

pub trait Provider {
    /// Pulumi Provider ID is the combination of URN and ID. It is used when creating a resource.
    fn get_provider_id(&self) -> Output<String>;
}

impl<T: Provider> From<&T> for Output<String> {
    fn from(provider: &T) -> Self {
        provider.get_provider_id()
    }
}

#[derive(Default, Builder, Clone)]
pub struct CustomResourceOptions {
    #[builder(with = |p: &impl Provider| { p.get_provider_id() })]
    pub provider: Option<Output<String>>,
}

pub type FunctionContext = Box<dyn Fn(Value) -> Value + Send>;

pub struct Output<T> {
    inner: integration::Output<FunctionContext>,
    phantom: PhantomData<T>,
    runtime: Rc<Runtime>,
}

impl<T> Output<T> {
    pub fn map<B, F>(&self, f: F) -> Output<B>
    where
        F: Fn(T) -> B + Send + 'static,
        T: DeserializeOwned,
        B: Serialize,
    {
        let function: FunctionContext = Box::new(move |v: Value| {
            let t: T = from_value(v).unwrap();
            let b = f(t);
            to_value(b).unwrap()
        });
        let res = self.runtime.block_on(self.inner.map(function));
        Output {
            inner: res,
            phantom: PhantomData,
            runtime: self.runtime.clone(),
        }
    }
    pub fn add_to_export(&self, key: &str) {
        self.runtime
            .block_on(self.inner.add_export(FieldName::from(key.to_string())));
    }
    pub fn combine<RESULT>(&self, others: &[&Output<()>]) -> Output<RESULT> {
        let outputs = others.iter().map(|o| &o.inner).collect::<Vec<_>>();
        let combined = self.runtime.block_on(self.inner.combine(&outputs));
        Output {
            inner: combined,
            phantom: PhantomData,
            runtime: self.runtime.clone(),
        }
    }
    pub fn drop_type(self) -> Output<()> {
        unsafe { self.transmute::<()>() }
    }
    unsafe fn transmute<F>(self) -> Output<F> {
        Output {
            inner: self.inner,
            phantom: PhantomData,
            runtime: self.runtime,
        }
    }
}

impl<T> Clone for Output<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            phantom: PhantomData,
            runtime: self.runtime.clone(),
        }
    }
}

pub struct CompositeOutput {
    inner: integration::RegisterResourceOutput<FunctionContext>,
    runtime: Rc<Runtime>,
}
impl CompositeOutput {
    pub fn get_field<T>(&self, key: &str) -> Output<T>
    where
        T: DeserializeOwned,
    {
        let res = self
            .runtime
            .block_on(self.inner.get_field(FieldName::from(key.to_string())));
        Output {
            inner: res,
            phantom: PhantomData,
            runtime: self.runtime.clone(),
        }
    }

    pub fn get_urn(&self) -> Output<String> {
        let res = self.runtime.block_on(self.inner.get_urn());
        Output {
            inner: res,
            phantom: PhantomData,
            runtime: self.runtime.clone(),
        }
    }

    pub fn get_id(&self) -> Output<String> {
        let res = self.runtime.block_on(self.inner.get_id());
        Output {
            inner: res,
            phantom: PhantomData,
            runtime: self.runtime.clone(),
        }
    }

    pub fn get_provider_id(&self) -> Output<String> {
        let res = self.runtime.block_on(self.inner.get_provider_id());
        Output {
            inner: res,
            phantom: PhantomData,
            runtime: self.runtime.clone(),
        }
    }
}

pub struct Context {
    inner: integration::Context<FunctionContext>,
    runtime: Rc<Runtime>,
}
impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
impl Context {
    pub fn new() -> Self {
        let runtime = Runtime::new().unwrap();
        let context = runtime.block_on(integration::Context::new());
        Self {
            inner: context,
            runtime: Rc::new(runtime),
        }
    }
    pub fn finish(&self) {
        self.runtime.block_on(
            pulumi_gestalt_rust_integration::finish::finish_lambdas_sequentially(&self.inner),
        )
    }
    pub fn new_output<T: Serialize>(&self, value: &T) -> Output<T> {
        let json_value = to_value(value).unwrap();
        Output {
            inner: self.inner.create_output(json_value, false),
            phantom: PhantomData,
            runtime: self.runtime.clone(),
        }
    }
    pub fn new_secret<T: Serialize>(&self, value: &T) -> Output<T> {
        let json_value = to_value(value).unwrap();
        Output {
            inner: self.inner.create_output(json_value, true),
            phantom: PhantomData,
            runtime: self.runtime.clone(),
        }
    }
    pub fn register_resource(
        &self,
        request: RegisterResourceRequest<Output<()>>,
    ) -> CompositeOutput {
        let mut inputs = HashMap::new();
        for object in request.object {
            inputs.insert(
                FieldName::from(object.name.clone()),
                object.value.inner.clone(),
            );
        }
        let provider = request
            .options
            .as_ref()
            .and_then(|o| o.provider.as_ref())
            .map(|p| p.inner.clone());

        let result = self.runtime.block_on(self.inner.register_resource(
            integration::RegisterResourceRequest {
                r#type: request.type_.clone(),
                name: request.name.clone(),
                version: request.version.clone(),
                inputs,
                provider,
            },
        ));
        CompositeOutput {
            inner: result,
            runtime: self.runtime.clone(),
        }
    }
    pub fn invoke_resource(&self, request: InvokeResourceRequest<Output<()>>) -> CompositeOutput {
        let mut inputs = HashMap::new();
        for object in request.object {
            inputs.insert(
                FieldName::from(object.name.clone()),
                object.value.inner.clone(),
            );
        }
        let result = self.runtime.block_on(self.inner.invoke_resource(
            integration::InvokeResourceRequest {
                token: request.token.clone(),
                version: request.version.clone(),
                inputs,
            },
        ));
        CompositeOutput {
            inner: result,
            runtime: self.runtime.clone(),
        }
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
            Some(ConfigValue::Secret(sec)) => Ok(Output {
                inner: sec,
                phantom: PhantomData,
                runtime: self.runtime.clone(),
            }),
            Some(ConfigValue::PlainText(_)) => {
                bail!("Config `{full_key}` is plaintext and cannot be read as secret")
            }
            None => {
                bail!("Config `{full_key}` does not exist")
            }
        }
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
}

pub struct RegisterResourceRequest<'a, OUTPUT> {
    pub type_: String,
    pub name: String,
    pub version: String,
    pub object: &'a [ResourceRequestObjectField<'a, OUTPUT>],
    pub options: Option<CustomResourceOptions>,
}
pub struct InvokeResourceRequest<'a, OUTPUT> {
    pub token: String,
    pub version: String,
    pub object: &'a [ResourceRequestObjectField<'a, OUTPUT>],
}
pub struct ResourceRequestObjectField<'a, OUTPUT> {
    pub name: String,
    pub value: &'a OUTPUT,
}
