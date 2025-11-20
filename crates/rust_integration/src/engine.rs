use pulumi_gestalt_core as core;
use pulumi_gestalt_domain::FieldName;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Context<FunctionContext> {
    pub(crate) inner: Arc<Mutex<core::Engine<FunctionContext>>>,
}

pub struct Output<FunctionContext> {
    pub(crate) inner: core::RawOutput,
    pub(crate) engine: Arc<Mutex<core::Engine<FunctionContext>>>,
}

pub struct RegisterResourceOutput<FunctionContext> {
    pub(crate) inner: core::RegisterResourceOutput,
    pub(crate) engine: Arc<Mutex<core::Engine<FunctionContext>>>,
}

pub struct RegisterResourceRequest {
    pub r#type: String,
    pub name: String,
    pub inputs: HashMap<FieldName, core::RawOutput>,
    pub version: String,
}

pub struct InvokeResourceRequest {
    pub token: String,
    pub inputs: HashMap<FieldName, core::RawOutput>,
    pub version: String,
}

impl<T> Context<T> {
    pub(crate) fn new(inner: core::Engine<T>) -> Self {
        Self { inner: Arc::new(Mutex::new(inner)) }
    }

    pub fn add_output(&self, field_name: FieldName, output_id: core::RawOutput) {
        self.inner.lock().unwrap().add_output(field_name, output_id)
    }

    pub fn register_resource(
        &self,
        args: RegisterResourceRequest
    ) -> RegisterResourceOutput<T> {
        let inner = self.inner.lock().unwrap().create_register_resource_node(
            args.r#type,
            args.name,
            args.inputs,
            args.version,
        );
        RegisterResourceOutput {
            inner,
            engine: Arc::clone(&self.inner),
        }
    }

    pub fn invoke_resource(
        &self,
        args: InvokeResourceRequest
    ) -> RegisterResourceOutput<T> {
        let inner = self.inner.lock().unwrap().create_resource_invoke_node(
            args.token,
            args.inputs,
            args.version,
        );
        RegisterResourceOutput {
            inner,
            engine: Arc::clone(&self.inner),
        }
    }

    pub fn create_native_function_node(
        &self,
        function_context: T,
        source: core::RawOutput,
    ) -> core::RawOutput {
        self.inner.lock().unwrap().create_native_function_node(function_context, source)
    }

    pub fn create_combine_outputs(&self, outputs: Vec<core::RawOutput>) -> core::RawOutput {
        self.inner.lock().unwrap().create_combine_outputs(outputs)
    }

    pub fn create_output(value: Value, secret: bool) -> core::RawOutput {
        core::Engine::<T>::create_done_node(value, secret)
    }

    pub fn get_config_value(&self, name: Option<&str>, key: &str) -> Option<core::ConfigValue> {
        self.inner.lock().unwrap().get_config_value(name, key)
    }

    pub async fn run(&mut self) -> Option<core::NativeFunctionRequest<T>> {
        self.inner.lock().unwrap().run().await
    }
}

impl<T> Output<T> {
    pub fn map(&self, func: T) -> Self {
        let raw_output = self.engine.lock().unwrap().create_native_function_node(
            func,
            self.inner.clone(),
        );
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.engine),
        }
    }
    
    pub fn add_export(&self, key: FieldName) {
        self.engine.lock().unwrap().add_output(key.into(), self.inner.clone());
    }
}

impl<T> RegisterResourceOutput<T> {
    
    pub fn get_field(&self, field_name: FieldName) -> Output<T> {
        let raw_output = core::Engine::<T>::create_extract_field(field_name, self.inner.clone());
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.engine),
        }
    }
    
}