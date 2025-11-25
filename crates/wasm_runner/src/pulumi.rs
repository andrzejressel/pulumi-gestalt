use crate::pulumi::runner::PulumiGestalt;
use anyhow::Error;
use log::info;
use pulumi_gestalt_wit::bindings_runner as runner;
use pulumi_gestalt_wit::bindings_runner::component::pulumi_gestalt::context::{
    CompositeOutput, ConfigValue, Context, FunctionInvocationRequest, FunctionInvocationResult,
    HostContext, Output,
};
use pulumi_gestalt_wit::bindings_runner::component::pulumi_gestalt::output_interface::{
    HostCompositeOutput, HostOutput,
};
use pulumi_gestalt_wit::bindings_runner::component::pulumi_gestalt::{
    context, output_interface, types,
};
use pulumi_gestalt_wit::bindings_runner::{
    SingleThreadedCompositeOutput, SingleThreadedContext, SingleThreadedOutput,
};
use std::collections::HashMap;
use std::path::Path;
use wasmtime::Store;
use wasmtime::component::{Component, HasSelf, Linker, Resource, ResourceTable};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

pub struct Pulumi {
    plugin: PulumiGestalt,
    store: Store<SimplePluginCtx>,
}

struct SimplePluginCtx {
    table: ResourceTable,
    context: WasiCtx,
    my_state: MyState,
}

struct MyState {
    table: ResourceTable,
}

impl HostContext for MyState {
    async fn new(&mut self) -> anyhow::Result<Resource<Context>> {
        let engine = pulumi_gestalt_rust_integration::Context::new().await;
        let context = SingleThreadedContext::new(engine);
        let id = self.table.push(context)?;
        Ok(id)
    }

    async fn create_output(
        &mut self,
        context: Resource<Context>,
        value: String,
        secret: bool,
    ) -> anyhow::Result<Resource<Output>> {
        assert!(!context.owned());
        let context = self.table.get_mut(&context)?;
        let value = serde_json::from_str(&value).unwrap();
        let output = context.engine.create_output(value, secret);
        let output = SingleThreadedOutput::new(output);
        let id = self.table.push(output)?;
        Ok(id)
    }

    async fn register_resource(
        &mut self,
        context: Resource<Context>,
        request: context::RegisterResourceRequest,
    ) -> anyhow::Result<Resource<CompositeOutput>> {
        assert!(!context.owned());
        let table = &self.table;
        let context = table.get(&context)?;

        let mut inputs = HashMap::new();
        for field in request.object {
            let output = table.get(&field.value)?;
            inputs.insert(field.name.clone().into(), output.output.clone());
        }

        let result = context.engine.register_resource(
            pulumi_gestalt_rust_integration::RegisterResourceRequest {
                r#type: request.type_,
                name: request.name,
                inputs,
                version: request.version,
            }
        );

        let output = SingleThreadedCompositeOutput::new(result);
        let id = self.table.push(output)?;
        Ok(id)
    }

    fn invoke_resource(
        &mut self,
        context: Resource<Context>,
        request: context::ResourceInvokeRequest,
    ) -> anyhow::Result<Resource<CompositeOutput>> {
        assert!(!context.owned());
        let table = &mut self.table;

        let mut inputs = HashMap::new();
        for field in request.object {
            let output = table.get(&field.value)?;
            inputs.insert(field.name.clone().into(), output.output.clone());
        }

        let context = table.get(&context)?;

        let result = context.engine.invoke_resource(
            pulumi_gestalt_rust_integration::InvokeResourceRequest {
                token: request.token,
                inputs,
                version: request.version,
            }
        );

        let output = SingleThreadedCompositeOutput::new(result);
        let id = self.table.push(output)?;
        Ok(id)
    }

    async fn finish(
        &mut self,
        self_: Resource<Context>,
        functions: Vec<FunctionInvocationResult>,
    ) -> anyhow::Result<Vec<FunctionInvocationRequest>> {
        assert!(!self_.owned());
        
        let context = self.table.get(&self_)?;
        
        let result = context.engine.finish().await;
        
        match result {
            None => Ok(Vec::new()),
            Some(request) => {
                // Process the single request
                let v = serde_json::to_string(&request.data).unwrap();
                let output = context.engine.create_native_function_node(
                    request.context,
                    // We need a dummy output here - this API doesn't quite match
                    // Will need refactoring for proper async support
                    context.engine.create_output(serde_json::Value::Null, false)
                );
                let output = SingleThreadedOutput::new(output);
                let id = self.table.push(output)?;
                
                // Send back the result (not implemented - needs mailbox)
                // request.return_mailbox.send(...).unwrap();
                
                Ok(vec![FunctionInvocationRequest {
                    id,
                    function_name: "placeholder".to_string(), // Need to extract from context
                    value: v,
                }])
            }
        }
    }

    async fn get_config(
        &mut self,
        context: Resource<Context>,
        name: Option<String>,
        key: String,
    ) -> anyhow::Result<Option<ConfigValue>> {
        assert!(!context.owned());
        let context = self.table.get_mut(&context)?;
        let result = context.engine.get_config_value(name.as_deref(), &key);
        let result = result.map(|value| match value {
            pulumi_gestalt_rust_integration::ConfigValue::PlainText(pt) => ConfigValue::Plaintext(pt),
            pulumi_gestalt_rust_integration::ConfigValue::Secret(s) => {
                let output = SingleThreadedOutput::new(s);
                let id = self.table.push(output).unwrap();
                ConfigValue::Secret(id)
            }
        });
        Ok(result)
    }

    async fn drop(&mut self, context: Resource<Context>) -> anyhow::Result<()> {
        assert!(context.owned());
        self.table.delete(context)?;
        Ok(())
    }
}

impl types::Host for MyState {}

impl context::Host for MyState {}

impl HostOutput for MyState {
    async fn map(
        &mut self,
        self_: Resource<SingleThreadedOutput>,
        function_name: String,
    ) -> anyhow::Result<Resource<SingleThreadedOutput>> {
        let table = &mut self.table;
        let st_output = table.get(&self_)?;
        let output = st_output.output.map(function_name);
        let output = SingleThreadedOutput::new(output);
        let id = table.push(output)?;
        Ok(id)
    }

    async fn clone(
        &mut self,
        self_: Resource<output_interface::Output>,
    ) -> anyhow::Result<Resource<output_interface::Output>> {
        assert!(!self_.owned());
        let st_output = self.table.get(&self_)?;
        let output = st_output.output.clone();
        let output = SingleThreadedOutput::new(output);
        let id = self.table.push(output)?;
        Ok(id)
    }

    async fn combine(
        &mut self,
        self_: Resource<output_interface::Output>,
        outputs: Vec<Resource<output_interface::Output>>,
    ) -> anyhow::Result<Resource<output_interface::Output>> {
        assert!(!self_.owned());
        let table = &self.table;
        let st_output = table.get(&self_)?;

        let mut output_refs = Vec::new();
        for output_res in &outputs {
            let output = table.get(output_res)?;
            output_refs.push(&output.output);
        }

        let result = st_output.output.combine(&output_refs);
        let output = SingleThreadedOutput::new(result);
        let id = self.table.push(output)?;
        Ok(id)
    }

    async fn add_to_export(
        &mut self,
        self_: Resource<output_interface::Output>,
        name: String,
    ) -> anyhow::Result<()> {
        assert!(!self_.owned());
        let table = &self.table;
        let output = table.get(&self_)?;
        output.output.add_export(name.into());
        Ok(())
    }

    async fn drop(&mut self, rep: Resource<output_interface::Output>) -> anyhow::Result<()> {
        assert!(rep.owned());
        self.table.delete(rep)?;
        Ok(())
    }
}

impl HostCompositeOutput for MyState {
    async fn get_field(
        &mut self,
        self_: Resource<output_interface::CompositeOutput>,
        field_name: String,
    ) -> anyhow::Result<Resource<output_interface::Output>> {
        assert!(!self_.owned());
        let composite_output = self.table.get(&self_)?;
        let output = composite_output.output.get_field(field_name.into());
        let output = SingleThreadedOutput::new(output);
        let id = self.table.push(output)?;
        Ok(id)
    }

    async fn drop(&mut self, rep: Resource<output_interface::CompositeOutput>) -> anyhow::Result<()> {
        assert!(rep.owned());
        self.table.delete(rep)?;
        Ok(())
    }
}

impl output_interface::Host for MyState {}

impl WasiView for SimplePluginCtx {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.context,
            table: &mut self.table,
        }
    }
}

impl Pulumi {
    pub fn create(program: &Path) -> Result<Pulumi, Error> {
        let mut engine_config = wasmtime::Config::new();
        engine_config.wasm_component_model(true);
        // engine_config.async_support(true);
        engine_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        engine_config.debug_info(true);

        let engine = wasmtime::Engine::new(&engine_config)?;

        let mut linker: Linker<SimplePluginCtx> = Linker::new(&engine);
        PulumiGestalt::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| &mut state.my_state)?;

        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

        let table = ResourceTable::new();
        let table2 = ResourceTable::new();

        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .inherit_env()
            .build();

        let mut store = Store::new(
            &engine,
            SimplePluginCtx {
                table,
                context: wasi_ctx,
                my_state: MyState { table: table2 },
            },
        );

        info!("Creating Wasm component");
        let component = Component::from_file(&engine, program)?;
        info!("Instantiating Wasm component");
        let plugin = PulumiGestalt::instantiate(&mut store, &component, &linker)?;
        info!("Wasm component instantiated");

        Ok(Pulumi { plugin, store })
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.plugin
            .component_pulumi_gestalt_pulumi_main()
            .call_main(&mut self.store)?;

        Ok(())
    }
}
