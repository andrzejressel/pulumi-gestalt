use futures::future::{BoxFuture, Shared};
use pulumi_gestalt_core::{
    Engine, FunctionName, NativeFunctionRequest, RawOutput, RegisterResourceOutput,
};
use pulumi_gestalt_domain::FieldName;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use futures::FutureExt;
use uuid::Uuid;
use crate::get_engine;

pub struct Output {
    output_id: RawOutput,
    engine: Arc<Mutex<InnerPulumiEngine>>,
}

static_assertions::assert_impl_all!(Output: Send);

pub enum ConfigValue {
    PlainText(String),
    Secret(Output),
}

pub struct ObjectField<'a> {
    pub name: String,
    pub value: &'a Output,
}


static_assertions::assert_impl_all!(ObjectField: Send);

pub struct CompositeOutput {
    output_id: RegisterResourceOutput,
    engine: Arc<Mutex<InnerPulumiEngine>>,
}


static_assertions::assert_impl_all!(CompositeOutput: Send);

type FunctionType = dyn Fn(String) -> Shared<BoxFuture<'static, String>> + Send;

pub(crate) struct InnerPulumiEngine {
    engine: Engine,
    functions: HashMap<FunctionName, Box<FunctionType>>,
}


static_assertions::assert_impl_all!(InnerPulumiEngine: Send);

pub struct Context {
    inner: Arc<Mutex<InnerPulumiEngine>>,
}


static_assertions::assert_impl_all!(Context: Send);

pub struct RegisterResourceRequest<'a> {
    pub type_: String,
    pub name: String,
    pub version: String,
    pub inputs: &'a [ObjectField<'a>],
}

pub struct InvokeResourceRequest<'a> {
    pub token: String,
    pub version: String,
    pub inputs: &'a [ObjectField<'a>],
}

impl Context {
    pub async fn create_context() -> Context {
        let engine = get_engine().await;
        let inner = InnerPulumiEngine {
            engine,
            functions: HashMap::new(),
        };
        Context {
            inner: Arc::new(Mutex::new(inner))
        }
    }

    pub fn create_output(&self, value: String, secret: bool) -> Output {
        let value = serde_json::from_str(&value).unwrap();
        let output_id = self
            .inner
            .lock()
            .unwrap()
            .engine
            .create_done_node(value, secret);
        Output {
            output_id,
            engine: Arc::clone(&self.inner),
        }
    }

    pub fn register_resource(&self, request: RegisterResourceRequest) -> CompositeOutput {
        let type_ = request.type_;
        let name = request.name;
        let version = request.version;

        let mut objects_map = HashMap::new();
        for object in request.inputs {
            objects_map.insert(
                FieldName::from(&object.name),
                object.value.output_id.clone(),
            );
        }

        let output_id = self
            .inner
            .lock()
            .unwrap()
            .engine
            .create_register_resource_node(type_, name, objects_map, version);

        CompositeOutput {
            output_id,
            engine: Arc::clone(&self.inner),
        }
    }

    pub fn invoke_resource(&self, request: InvokeResourceRequest) -> CompositeOutput {
        let mut objects_map = HashMap::new();
        for object in request.inputs {
            objects_map.insert(
                FieldName::from(&object.name),
                object.value.output_id.clone(),
            );
        }

        let output_id = self
            .inner
            .lock()
            .unwrap()
            .engine
            .create_resource_invoke_node(request.token, objects_map, request.version);

        CompositeOutput {
            output_id,
            engine: Arc::clone(&self.inner),
        }
    }

    pub async fn finish(&self) {
        let mut inner = self.inner.lock().unwrap();
        loop {
            let result = inner.engine.run().await;

            match result {
                None => break,
                Some(NativeFunctionRequest {
                         function_name,
                         data,
                         return_mailbox,
                     }) => {
                    let function = inner.functions.get(&function_name).unwrap();
                    let s = data.to_string();

                    let result = function(s).await;

                    let result = serde_json::from_str(&result).unwrap();

                    return_mailbox.send(result).unwrap();
                }
            }
        }
    }

    pub fn get_config_value(&self, name: Option<&str>, key: &str) -> Option<ConfigValue> {
        let pulumi_engine = &self.inner;

        match pulumi_engine
            .lock()
            .unwrap()
            .engine
            .get_config_value(name, key)
        {
            None => None,
            Some(pulumi_gestalt_core::ConfigValue::PlainText(value)) => {
                Some(ConfigValue::PlainText(value))
            }
            Some(pulumi_gestalt_core::ConfigValue::Secret(output_id)) => {
                let output = Output {
                    output_id,
                    engine: Arc::clone(pulumi_engine),
                };
                Some(ConfigValue::Secret(output))
            }
        }
    }
}

impl Output {
    pub fn add_export(&self, name: String) {
        let pulumi_engine = &self.engine;
        let output_id = self.output_id.clone();
        pulumi_engine
            .lock()
            .unwrap()
            .engine
            .add_output(name.into(), output_id);
    }

    pub fn map<Fut: Future<Output = String> + Send + 'static, F: Fn(String) -> Fut + Send + 'static>(&self, function: F) -> Output {
        let output_id = &self.output_id;
        let function_uuid = Uuid::new_v4();
        let function_name: FunctionName = function_uuid.to_string().into();

        let mut inner = self.engine.lock().unwrap();

        let output = inner
            .engine
            .create_native_function_node(function_name.clone(), output_id.clone());
        let output = Output {
            output_id: output,
            engine: Arc::clone(&self.engine),
        };

        let f = move |data: String| {
            let fut = function(data);
            fut.boxed().shared()
        };

        inner.functions.insert(function_name, Box::new(f));

        output
    }

    pub fn combine(outputs: &[&Output]) -> Output {
        let pulumi_engine = &self.engine;
        let mut outputs = Vec::with_capacity(others.len() + 1);
        outputs.push(self.output_id.clone());
        for o in others {
            outputs.push(o.output_id.clone());
        }

        let output = pulumi_engine
            .lock()
            .unwrap()
            .engine
            .create_combine_outputs(outputs);

        Output {
            output_id: output,
            engine: Arc::clone(pulumi_engine),
        }
    }
}

impl CompositeOutput {
    pub fn get_field(&self, field_name: String) -> Output {
        let pulumi_engine = &self.engine;
        let output_id = &self.output_id;

        let output = pulumi_engine
            .lock()
            .unwrap()
            .engine
            .create_extract_field(field_name.into(), output_id.clone());

        Output {
            output_id: output,
            engine: Arc::clone(pulumi_engine),
        }
    }
}
