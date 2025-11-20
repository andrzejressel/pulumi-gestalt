use std::rc::Rc;
use std::collections::HashMap;
use pulumi_gestalt_core as core;
use pulumi_gestalt_domain::FieldName;
use serde_json::Value;

pub struct Engine<T> {
    pub(crate) inner: Rc<core::Engine<T>>,
}

pub struct Output<T> {
    pub(crate) inner: core::RawOutput,
    pub(crate) engine: Rc<core::Engine<T>>,
}

pub struct RegisterResourceOutput<T> {
    pub(crate) inner: core::RegisterResourceOutput,
    pub(crate) engine: Rc<core::Engine<T>>,
}

impl<T> Engine<T> {
    pub(crate) fn new(inner: core::Engine<T>) -> Self {
        Self { inner: Rc::new(inner) }
    }

    pub fn add_output(&self, field_name: FieldName, output_id: core::RawOutput) {
        self.inner.add_output(field_name, output_id)
    }

    pub fn create_register_resource_node(
        &self,
        r#type: String,
        name: String,
        inputs: HashMap<FieldName, core::RawOutput>,
        version: String,
    ) -> core::RegisterResourceOutput {
        self.inner.create_register_resource_node(r#type, name, inputs, version)
    }

    pub fn create_resource_invoke_node(
        &self,
        token: String,
        inputs: HashMap<FieldName, core::RawOutput>,
        version: String,
    ) -> core::RegisterResourceOutput {
        self.inner.create_resource_invoke_node(token, inputs, version)
    }

    pub fn create_native_function_node(
        &self,
        function_context: T,
        source: core::RawOutput,
    ) -> core::RawOutput {
        self.inner.create_native_function_node(function_context, source)
    }

    pub fn create_combine_outputs(&self, outputs: Vec<core::RawOutput>) -> core::RawOutput {
        self.inner.create_combine_outputs(outputs)
    }

    pub fn create_done_node(value: Value, secret: bool) -> core::RawOutput {
        core::Engine::<T>::create_done_node(value, secret)
    }

    pub fn get_config_value(&self, name: Option<&str>, key: &str) -> Option<core::ConfigValue> {
        self.inner.get_config_value(name, key)
    }

    pub async fn run(&mut self) -> Option<core::NativeFunctionRequest<T>> {
        // Note: This requires mutable access to inner which is behind Rc
        // This is a design issue that needs to be resolved
        todo!("Cannot mutate through Rc - needs RefCell or redesign")
    }
}

impl<T> RegisterResourceOutput<T> {
    
    pub fn extract_field(&self, field_name: FieldName) -> Output<Value> {
        let raw_output = Engine::<T>::create_extract_field(field_name, self.inner.clone());
        Output {
            inner: raw_output,
            engine: Rc::clone(&self.engine),
        }
    }
    
}