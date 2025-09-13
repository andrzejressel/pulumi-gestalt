mod native_pulumi_connector;
mod error;

pub use error::{IntegrationError, IntegrationResult};

use anyhow::Context as AnyHowContext;
use anyhow::Result;
use pulumi_gestalt_core::{
    Config, Engine, FieldName, ForeignFunctionToInvoke, FunctionName, OutputId, PulumiServiceImpl,
};
use pulumi_gestalt_schema::model;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone)]
pub struct Output {
    output_id: OutputId,
    engine: Rc<RefCell<InnerPulumiEngine>>,
}

pub enum ConfigValue {
    PlainText(String),
    Secret(Output),
}

pub struct ObjectField<'a> {
    pub name: String,
    pub value: &'a Output,
}

pub struct CompositeOutput {
    output_id: OutputId,
    engine: Rc<RefCell<InnerPulumiEngine>>,
}

pub(crate) struct InnerPulumiEngine {
    engine: Engine,
    functions: HashMap<FunctionName, Box<dyn Fn(String) -> String>>,
}

pub struct Context {
    inner: Rc<RefCell<InnerPulumiEngine>>,
    project_name: String,
}

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
    pub fn try_create_context() -> IntegrationResult<Context> {
        let engine = try_get_engine()?;
        let project_name = std::env::var("PULUMI_PROJECT")
            .map_err(|_| IntegrationError::MissingEnvironmentVariable("PULUMI_PROJECT".to_string()))?;
        let inner = InnerPulumiEngine {
            engine,
            functions: HashMap::new(),
        };
        Ok(Context {
            inner: Rc::new(RefCell::new(inner)),
            project_name,
        })
    }

    pub fn create_context() -> Context {
        // Unwrap at this public API boundary for backward compatibility
        Self::try_create_context().unwrap()
    }

    pub fn try_create_output(&self, value: String, secret: bool) -> IntegrationResult<Output> {
        let value: Value = serde_json::from_str(&value)?;
        let output_id = self
            .inner
            .deref()
            .borrow_mut()
            .engine
            .create_done_node(value, secret);
        Ok(Output {
            output_id,
            engine: Rc::clone(&self.inner),
        })
    }

    pub fn create_output(&self, value: String, secret: bool) -> Output {
        // Unwrap at this public API boundary for backward compatibility
        self.try_create_output(value, secret).unwrap()
    }

    pub fn register_resource(&self, request: RegisterResourceRequest) -> CompositeOutput {
        let type_ = request.type_;
        let name = request.name;
        let version = request.version;

        let mut objects_map = HashMap::new();
        for object in request.inputs {
            objects_map.insert(FieldName::from(&object.name), object.value.output_id);
        }

        let output_id = self
            .inner
            .deref()
            .borrow_mut()
            .engine
            .create_register_resource_node(type_, name, objects_map, version);

        CompositeOutput {
            output_id,
            engine: Rc::clone(&self.inner),
        }
    }

    pub fn invoke_resource(&self, request: InvokeResourceRequest) -> CompositeOutput {
        let mut objects_map = HashMap::new();
        for object in request.inputs {
            objects_map.insert(FieldName::from(&object.name), object.value.output_id);
        }

        let output_id = self
            .inner
            .deref()
            .borrow_mut()
            .engine
            .create_resource_invoke_node(request.token, objects_map, request.version);

        CompositeOutput {
            output_id,
            engine: Rc::clone(&self.inner),
        }
    }

    pub fn finish(&self) {
        self.finish_loop(HashMap::new());
    }

    fn try_finish_loop(&self, mut native_function_result: HashMap<OutputId, Value>) -> IntegrationResult<()> {
        let mut inner = self.inner.borrow_mut();
        loop {
            let result = inner.engine.run(native_function_result);

            match result {
                None => break,
                Some(functions_to_invoke) => {
                    native_function_result = HashMap::new();

                    for ForeignFunctionToInvoke {
                        output_id,
                        function_name,
                        value,
                    } in functions_to_invoke.iter()
                    {
                        let function = inner.functions.get(function_name)
                            .ok_or_else(|| IntegrationError::FunctionNotFound(function_name.clone().into()))?;
                        let s = value.to_string();

                        let result = function(s);

                        let result: Value = serde_json::from_str(&result)?;

                        native_function_result.insert(*output_id, result);
                    }
                }
            }
        }
        Ok(())
    }

    fn finish_loop(&self, native_function_result: HashMap<OutputId, Value>) {
        // Unwrap at this internal boundary for backward compatibility
        self.try_finish_loop(native_function_result).unwrap()
    }

    pub fn get_config_value(&self, name: Option<&str>, key: &str) -> Option<ConfigValue> {
        let pulumi_engine = &self.inner;
        let name = name.unwrap_or(&self.project_name);

        match pulumi_engine
            .borrow_mut()
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
                    engine: Rc::clone(pulumi_engine),
                };
                Some(ConfigValue::Secret(output))
            }
        }
    }
}

impl Output {
    pub fn add_export(&self, name: String) {
        let pulumi_engine = &self.engine;
        let output_id = self.output_id;
        pulumi_engine
            .deref()
            .borrow_mut()
            .engine
            .add_output(name.into(), output_id);
    }

    pub fn map(&self, function: Box<dyn Fn(String) -> String>) -> Output {
        let output_id = self.output_id;
        let function_uuid = Uuid::new_v4();
        let function_name: FunctionName = function_uuid.to_string().into();

        let mut inner = self.engine.borrow_mut();

        let output = inner
            .engine
            .create_native_function_node(function_name.clone(), output_id);
        let output = Output {
            output_id: output,
            engine: Rc::clone(&self.engine),
        };

        inner.functions.insert(function_name, function);

        output
    }

    pub fn combine(&self, others: &[&Output]) -> Output {
        let pulumi_engine = &self.engine;
        let mut outputs = Vec::with_capacity(others.len() + 1);
        outputs.push(self.output_id);
        for o in others {
            outputs.push(o.output_id);
        }

        let output = pulumi_engine
            .borrow_mut()
            .engine
            .create_combine_outputs(outputs);

        Output {
            output_id: output,
            engine: Rc::clone(pulumi_engine),
        }
    }
}

impl CompositeOutput {
    pub fn get_field(&self, field_name: String) -> Output {
        let pulumi_engine = &self.engine;
        let output_id = &self.output_id;

        let output = pulumi_engine
            .borrow_mut()
            .engine
            .create_extract_field(field_name.into(), *output_id);

        Output {
            output_id: output,
            engine: Rc::clone(pulumi_engine),
        }
    }
}

pub fn try_get_engine() -> IntegrationResult<Engine> {
    let pulumi_engine_url = std::env::var("PULUMI_ENGINE")
        .map_err(|_| IntegrationError::MissingEnvironmentVariable("PULUMI_ENGINE".to_string()))?;
    let pulumi_monitor_url = std::env::var("PULUMI_MONITOR")
        .map_err(|_| IntegrationError::MissingEnvironmentVariable("PULUMI_MONITOR".to_string()))?;
    let pulumi_stack = std::env::var("PULUMI_STACK")
        .map_err(|_| IntegrationError::MissingEnvironmentVariable("PULUMI_STACK".to_string()))?;
    let pulumi_project = std::env::var("PULUMI_PROJECT")
        .map_err(|_| IntegrationError::MissingEnvironmentVariable("PULUMI_PROJECT".to_string()))?;
    
    let in_preview = match std::env::var("PULUMI_DRY_RUN") {
        Ok(preview) if preview == "true" => true,
        Ok(preview) if preview == "false" => false,
        _ => false,
    };

    let native_pulumi_connector = native_pulumi_connector::NativePulumiConnector::new(
        pulumi_monitor_url,
        pulumi_engine_url,
        pulumi_project,
        pulumi_stack,
    );

    let config = Config::from_env_vars()
        .context("Failed to create config instance")?;

    Ok(Engine::new(
        PulumiServiceImpl::new(native_pulumi_connector, in_preview),
        config,
    ))
}

pub fn get_engine() -> Engine {
    // Unwrap at this public API boundary for backward compatibility
    try_get_engine().unwrap()
}

/// Requires `pulumi` CLI to be installed and available in PATH
/// Modules for provider can be found in Pulumi registry on left side with (M) icon:
/// - [AWS](https://www.pulumi.com/registry/packages/aws/)
/// - [Azure](https://www.pulumi.com/registry/packages/azure/)
/// - [GCP](https://www.pulumi.com/registry/packages/gcp/)
///
/// Empty modules list means that no modules are used.
/// To use all modules, pass `None` as `modules` argument.
pub fn get_schema(
    provider_name: &str,
    provider_version: &str,
    modules: Option<&[&str]>,
) -> Result<model::Package> {
    pulumi_gestalt_schema::get_schema(provider_name, provider_version, modules)
}
