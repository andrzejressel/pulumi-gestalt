use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, from_value, to_value};
use std::marker::PhantomData;
use tokio::runtime::Runtime;
use pulumi_gestalt_rust_integration as integration;
use pulumi_gestalt_rust_integration::FieldName;

pub type FunctionContext = Box<dyn Fn(Value) -> Value + Send>;

pub struct Output<T> {
    inner: integration::Output<FunctionContext>,
    phantom: PhantomData<T>,
}

impl<T> Output<T> {
    pub fn map<B, F>(&self, f: F) -> Output<B>
    where
        F: Fn(T) -> B + Send + 'static,
        T: DeserializeOwned,
        B: Serialize,
    {
        // Closure converting Value JSON -> T, applying user function, returning Value JSON
        let function: FunctionContext = Box::new(move |v: Value| {
            let t: T = from_value(v).unwrap();
            let b = f(t);
            to_value(b).unwrap()
        });
        let res = block_on(self.inner.map(function));
        Output {
            inner: res,
            phantom: PhantomData,
        }
    }
    pub fn add_to_export(&self, key: &str) {
        block_on(self.inner.add_export(FieldName::from(key.to_string())));
    }
    pub fn combine<RESULT>(&self, others: &[&Output<()>]) -> Output<RESULT> {
        let mut all: Vec<&integration::Output<FunctionContext>> =
            Vec::with_capacity(others.len() + 1);
        all.push(&self.inner);
        for o in others {
            all.push(&o.inner);
        }
        let combined = block_on(self.inner.combine(&all));
        Output {
            inner: combined,
            phantom: PhantomData,
        }
    }
    pub fn drop_type(self) -> Output<()> {
        unsafe { self.transmute::<()>() }
    }
    unsafe fn transmute<F>(self) -> Output<F> {
        Output {
            inner: self.inner,
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for Output<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            phantom: PhantomData,
        }
    }
}

pub struct CompositeOutput {
    inner: integration::RegisterResourceOutput<FunctionContext>,
}
impl CompositeOutput {
    pub fn get_field<T>(&self, key: &str) -> Output<T>
    where
        T: DeserializeOwned,
    {
        let res = block_on(self.inner.get_field(FieldName::from(key.to_string())));
        Output {
            inner: res,
            phantom: PhantomData,
        }
    }
}

pub struct Context {
    inner: integration::Context<FunctionContext>,
    runtime: Runtime
}
impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
impl Context {
    pub fn new() -> Self {
        let runtime = Runtime::new().unwrap();
        Self {
            inner: runtime.block_on(integration::Context::new()),
            runtime
        }
    }
    pub fn finish(&self) {
        loop {
            let result = block_on(self.inner.finish());
            match result {
                None => break,
                Some(NativeFunctionRequest {
                    context,
                    data,
                    return_mailbox,
                }) => {
                    let value = context(data);
                    return_mailbox.send(value).unwrap();
                }
            }
        }
    }
    pub fn new_output<T: Serialize>(&self, value: &T) -> Output<T> {
        let json_value = to_value(value).unwrap();
        Output {
            inner: self.inner.create_output(json_value, false),
            phantom: PhantomData,
        }
    }
    pub fn new_secret<T: Serialize>(&self, value: &T) -> Output<T> {
        let json_value = to_value(value).unwrap();
        Output {
            inner: self.inner.create_output(json_value, true),
            phantom: PhantomData,
        }
    }
    pub fn register_resource(
        &self,
        request: RegisterResourceRequest<Output<()>>,
    ) -> CompositeOutput {
        let mut inputs = std::collections::HashMap::new();
        for object in request.object {
            inputs.insert(
                FieldName::from(object.name.clone()),
                object.value.inner.clone(),
            );
        }
        let result = block_on(
            self.inner
                .register_resource(integration::RegisterResourceRequest {
                    r#type: request.type_.clone(),
                    name: request.name.clone(),
                    version: request.version.clone(),
                    inputs,
                }),
        );
        CompositeOutput { inner: result }
    }
    pub fn invoke_resource(&self, request: InvokeResourceRequest<Output<()>>) -> CompositeOutput {
        let mut inputs = std::collections::HashMap::new();
        for object in request.object {
            inputs.insert(
                FieldName::from(object.name.clone()),
                object.value.inner.clone(),
            );
        }
        let result = block_on(
            self.inner
                .invoke_resource(integration::InvokeResourceRequest {
                    token: request.token.clone(),
                    version: request.version.clone(),
                    inputs,
                }),
        );
        CompositeOutput { inner: result }
    }
    pub fn get_config(&self, name: Option<&str>, key: &str) -> Option<ConfigValue<Output<String>>> {
        block_on(self.inner.get_config_value(name, key)).map(|v| match v {
            pulumi_gestalt_rust_integration::ConfigValue::PlainText(pt) => {
                ConfigValue::PlainText(pt)
            }
            pulumi_gestalt_rust_integration::ConfigValue::Secret(sec) => {
                ConfigValue::Secret(Output {
                    inner: sec,
                    phantom: PhantomData,
                })
            }
        })
    }
}

pub struct RegisterResourceRequest<'a, OUTPUT> {
    pub type_: String,
    pub name: String,
    pub version: String,
    pub object: &'a [ResourceRequestObjectField<'a, OUTPUT>],
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
pub enum ConfigValue<OUTPUT> {
    PlainText(String),
    Secret(OUTPUT),
}
