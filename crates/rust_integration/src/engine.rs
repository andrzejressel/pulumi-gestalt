use anyhow::Context as AnyhowContext;
use futures::lock::Mutex;
use pulumi_gestalt_core as core;
use pulumi_gestalt_core::{Config, Engine};
use pulumi_gestalt_domain::FieldName;
use pulumi_gestalt_grpc_connection::RealPulumiConnector;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Context<FunctionContext> {
    inner: Arc<Mutex<core::Engine<FunctionContext>>>,
}

pub struct Output<FunctionContext> {
    inner: core::RawOutput,
    engine: Arc<Mutex<core::Engine<FunctionContext>>>,
}

impl<T> Clone for Output<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            engine: Arc::clone(&self.engine),
        }
    }
}

pub struct RegisterResourceOutput<FunctionContext> {
    pub(crate) inner: core::RegisterResourceOutput,
    pub(crate) engine: Arc<Mutex<core::Engine<FunctionContext>>>,
}

pub struct RegisterResourceRequest<FunctionContext> {
    pub r#type: String,
    pub name: String,
    pub inputs: HashMap<FieldName, Output<FunctionContext>>,
    pub version: String,
}

pub struct InvokeResourceRequest<FunctionContext> {
    pub token: String,
    pub inputs: HashMap<FieldName, Output<FunctionContext>>,
    pub version: String,
}

pub enum ConfigValue<FunctionContext> {
    PlainText(String),
    Secret(Output<FunctionContext>),
}

impl<T> Context<T> {
    pub async fn new() -> Context<T> {
        let pulumi_engine_url = std::env::var("PULUMI_ENGINE").unwrap();
        let pulumi_monitor_url = std::env::var("PULUMI_MONITOR").unwrap();
        let pulumi_stack = std::env::var("PULUMI_STACK").unwrap();
        let pulumi_project = std::env::var("PULUMI_PROJECT").unwrap();
        let in_preview = match std::env::var("PULUMI_DRY_RUN") {
            Ok(preview) if preview == "true" => true,
            Ok(preview) if preview == "false" => false,
            _ => false,
        };

        let pulumi_connector = RealPulumiConnector::new(
            pulumi_monitor_url,
            pulumi_engine_url,
            pulumi_project,
            pulumi_stack,
            in_preview,
        )
        .await
        .context("Failed to create Pulumi connector")
        .unwrap();

        let config = Config::from_env_vars()
            .context("Failed to create config instance")
            .unwrap();

        Context {
            inner: Arc::new(Mutex::new(Engine::new(pulumi_connector, config))),
        }
    }

    pub async fn add_output(&self, field_name: FieldName, output: Output<T>) {
        self.inner.lock().await.add_output(field_name, output.inner)
    }

    pub async fn register_resource(
        &self,
        args: RegisterResourceRequest<T>,
    ) -> RegisterResourceOutput<T> {
        let inputs = args.inputs.into_iter().map(|(k, v)| (k, v.inner)).collect();
        let inner = self.inner.lock().await.create_register_resource_node(
            args.r#type,
            args.name,
            inputs,
            args.version,
        );
        RegisterResourceOutput {
            inner,
            engine: Arc::clone(&self.inner),
        }
    }

    pub async fn invoke_resource(
        &self,
        args: InvokeResourceRequest<T>,
    ) -> RegisterResourceOutput<T> {
        let inputs = args.inputs.into_iter().map(|(k, v)| (k, v.inner)).collect();
        let inner =
            self.inner
                .lock()
                .await
                .create_resource_invoke_node(args.token, inputs, args.version);
        RegisterResourceOutput {
            inner,
            engine: Arc::clone(&self.inner),
        }
    }

    pub async fn create_native_function_node(
        &self,
        function_context: T,
        source: Output<T>,
    ) -> Output<T> {
        let raw_output = self
            .inner
            .lock()
            .await
            .create_native_function_node(function_context, source.inner);
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.inner),
        }
    }

    pub async fn create_combine_outputs(&self, outputs: Vec<Output<T>>) -> Output<T> {
        let raw_outputs: Vec<core::RawOutput> = outputs.into_iter().map(|o| o.inner).collect();
        let raw_output = self.inner.lock().await.create_combine_outputs(raw_outputs);
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.inner),
        }
    }

    pub fn create_output(&self, value: Value, secret: bool) -> Output<T> {
        let raw_output = core::Engine::<T>::create_done_node(value, secret);
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.inner),
        }
    }

    pub async fn get_config_value(&self, name: Option<&str>, key: &str) -> Option<ConfigValue<T>> {
        self.inner
            .lock()
            .await
            .get_config_value(name, key)
            .map(|v| match v {
                core::ConfigValue::PlainText(s) => ConfigValue::PlainText(s),
                core::ConfigValue::Secret(o) => ConfigValue::Secret(Output {
                    inner: o,
                    engine: Arc::clone(&self.inner),
                }),
            })
    }

    pub async fn finish(&self) -> Option<core::NativeFunctionRequest<T>> {
        self.inner.lock().await.run().await
    }
}

impl<T> Output<T> {
    pub async fn map(&self, func: T) -> Self {
        let raw_output = self
            .engine
            .lock()
            .await
            .create_native_function_node(func, self.inner.clone());
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.engine),
        }
    }

    pub async fn combine(&self, others: &[&Output<T>]) -> Self {
        let mut all_outputs = vec![self.inner.clone()];
        for other in others {
            all_outputs.push(other.inner.clone());
        }
        let raw_output = self.engine.lock().await.create_combine_outputs(all_outputs);
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.engine),
        }
    }

    pub async fn add_export(&self, key: FieldName) {
        self.engine
            .lock()
            .await
            .add_output(key.into(), self.inner.clone());
    }
}

impl<T> RegisterResourceOutput<T> {
    pub async fn get_field(&self, field_name: FieldName) -> Output<T> {
        let raw_output = core::Engine::<T>::create_extract_field(field_name, self.inner.clone());
        Output {
            inner: raw_output,
            engine: Arc::clone(&self.engine),
        }
    }
}
