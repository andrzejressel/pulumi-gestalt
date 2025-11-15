use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;

use pulumi_gestalt_rust_integration as integration;

// Core types merged from former adapter crates
pub struct Output<T> {
    inner: integration::Output,
    phantom: PhantomData<T>,
}

impl<T> Output<T> {
    pub fn map<B, F>(&self, f: F) -> Output<B>
    where
        F: Fn(T) -> B + Send + 'static,
        T: DeserializeOwned,
        B: Serialize,
    {
        let function = move |v: String| {
            let v: T = serde_json::from_str(&v).unwrap();
            let v = f(v);
            serde_json::to_string(&v).unwrap()
        };
        let res = self.inner.map(Box::new(function));
        Output {
            inner: res,
            phantom: PhantomData,
        }
    }
    pub fn add_to_export(&self, key: &str) {
        self.inner.add_export(key.to_string());
    }
    pub fn combine<RESULT>(&self, others: &[&Output<()>]) -> Output<RESULT> {
        let all = others.iter().map(|o| &o.inner).collect::<Vec<_>>();
        let combined = self.inner.combine(&all);
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
    inner: integration::CompositeOutput,
}
impl CompositeOutput {
    pub fn get_field<T>(&self, key: &str) -> Output<T> {
        let res = self.inner.get_field(key.to_string());
        Output {
            inner: res,
            phantom: PhantomData,
        }
    }
}

pub struct Context {
    inner: integration::Context,
}
impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
impl Context {
    pub fn new() -> Self {
        Self {
            inner: integration::Context::create_context(),
        }
    }
    pub fn finish(&self) {
        self.inner.finish();
    }
    pub fn new_output<T: Serialize>(&self, value: &T) -> Output<T> {
        let json = serde_json::to_string(value).unwrap();
        Output {
            inner: self.inner.create_output(json, false),
            phantom: PhantomData,
        }
    }
    pub fn new_secret<T: Serialize>(&self, value: &T) -> Output<T> {
        let json = serde_json::to_string(value).unwrap();
        Output {
            inner: self.inner.create_output(json, true),
            phantom: PhantomData,
        }
    }
    pub fn register_resource(
        &self,
        request: RegisterResourceRequest<Output<()>>,
    ) -> CompositeOutput {
        let mut objects = Vec::new();
        for object in request.object {
            objects.push(integration::ObjectField {
                name: object.name.clone(),
                value: &object.value.inner,
            });
        }
        let result = self
            .inner
            .register_resource(integration::RegisterResourceRequest {
                type_: request.type_,
                name: request.name,
                version: request.version,
                inputs: &objects,
            });
        CompositeOutput { inner: result }
    }
    pub fn invoke_resource(&self, request: InvokeResourceRequest<Output<()>>) -> CompositeOutput {
        let mut objects = Vec::new();
        for object in request.object {
            objects.push(integration::ObjectField {
                name: object.name.clone(),
                value: &object.value.inner,
            });
        }
        let result = self
            .inner
            .invoke_resource(integration::InvokeResourceRequest {
                token: request.token,
                version: request.version,
                inputs: &objects,
            });
        CompositeOutput { inner: result }
    }
    pub fn get_config(&self, name: Option<&str>, key: &str) -> Option<ConfigValue<Output<String>>> {
        self.inner.get_config_value(name, key).map(|v| match v {
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
