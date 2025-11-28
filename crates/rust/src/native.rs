use pulumi_gestalt_rust_integration as integration;
use pulumi_gestalt_rust_integration::FieldName;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, from_value, to_value};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use tokio::runtime::Runtime;

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
        let result = self.runtime.block_on(self.inner.register_resource(
            integration::RegisterResourceRequest {
                r#type: request.type_.clone(),
                name: request.name.clone(),
                version: request.version.clone(),
                inputs,
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
    pub fn get_config(&self, name: Option<&str>, key: &str) -> Option<ConfigValue<Output<String>>> {
        self.runtime
            .block_on(self.inner.get_config_value(name, key))
            .map(|v| match v {
                pulumi_gestalt_rust_integration::ConfigValue::PlainText(pt) => {
                    ConfigValue::PlainText(pt)
                }
                pulumi_gestalt_rust_integration::ConfigValue::Secret(sec) => {
                    ConfigValue::Secret(Output {
                        inner: sec,
                        phantom: PhantomData,
                        runtime: self.runtime.clone(),
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
