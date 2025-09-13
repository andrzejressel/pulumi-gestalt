use crate::pulumi::runner::PulumiGestalt;
use anyhow::{Context as AnyhowContext, Error};
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
    fn new(&mut self) -> anyhow::Result<Resource<Context>> {
        let engine = pulumi_gestalt_rust_integration::try_get_engine()
            .map_err(|e| anyhow::anyhow!("Failed to create engine: {}", e))?;
        let project_name = std::env::var("PULUMI_PROJECT")
            .with_context(|| "PULUMI_PROJECT environment variable must be set")?;

        let context = SingleThreadedContext::new(engine, project_name);
        let id = self.table.push(context)?;
        Ok(id)
    }

    fn create_output(
        &mut self,
        context: Resource<Context>,
        value: String,
        secret: bool,
    ) -> anyhow::Result<Resource<Output>> {
        assert!(!context.owned());
        let context = self.table.get_mut(&context)?;
        let value: serde_json::Value = serde_json::from_str(&value)
            .with_context(|| "Failed to parse JSON value")?;
        let refcell = &context.engine;
        let output_id = refcell.borrow_mut().create_done_node(value, secret);
        let output = SingleThreadedOutput::new(output_id, context.engine.clone());
        let id = self.table.push(output)?;
        Ok(id)
    }

    fn register_resource(
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
            inputs.insert(field.name.clone().into(), output.output_id);
        }

        let result = context.engine.borrow_mut().create_register_resource_node(
            request.type_,
            request.name,
            inputs,
            request.version,
        );

        let output = SingleThreadedCompositeOutput::new(result, context.engine.clone());
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
            inputs.insert(field.name.clone().into(), output.output_id);
        }

        let context = table.get(&context)?;

        let result = context.engine.borrow_mut().create_resource_invoke_node(
            request.token,
            inputs,
            request.version,
        );

        let output = SingleThreadedCompositeOutput::new(result, context.engine.clone());
        let id = self.table.push(output)?;
        Ok(id)
    }

    fn finish(
        &mut self,
        self_: Resource<Context>,
        functions: Vec<FunctionInvocationResult>,
    ) -> anyhow::Result<Vec<FunctionInvocationRequest>> {
        assert!(!self_.owned());
        let table = &mut self.table;
        let mut function_invocations = HashMap::new();
        for function in functions {
            let v: serde_json::Value = serde_json::from_str(function.value.as_str())
                .with_context(|| "Failed to parse function result JSON")?;
            let output = function.id;
            let output = table.get(&output)?;
            function_invocations.insert(output.output_id, v);
        }

        let context = table.get_mut(&self_)?;
        let engine = context.engine.clone();

        let results = context
            .engine
            .borrow_mut()
            .run(function_invocations)
            .unwrap_or_default()
            .clone();

        let requests: anyhow::Result<Vec<FunctionInvocationRequest>> = results
            .into_iter()
            .map(|result| -> anyhow::Result<FunctionInvocationRequest> {
                let v = serde_json::to_string(&result.value)
                    .with_context(|| "Failed to serialize function result to JSON")?;
                let output = SingleThreadedOutput::new(result.output_id, engine.clone());
                let id = table.push(output)
                    .with_context(|| "Failed to store output in resource table")?;
                Ok(FunctionInvocationRequest {
                    id,
                    function_name: result.function_name.into(),
                    value: v,
                })
            })
            .collect();
        
        Ok(requests?)
    }

    fn get_config(
        &mut self,
        context: Resource<Context>,
        name: Option<String>,
        key: String,
    ) -> anyhow::Result<Option<ConfigValue>> {
        assert!(!context.owned());
        let context = self.table.get_mut(&context)?;
        let project_name = context.project_name.clone();
        let engine = &context.engine.clone();
        let result = engine
            .clone()
            .borrow_mut()
            .get_config_value(&name.unwrap_or(project_name), &key);
        let result = result.map(|value| -> anyhow::Result<ConfigValue> {
            match value {
                pulumi_gestalt_core::ConfigValue::PlainText(pt) => Ok(ConfigValue::Plaintext(pt.clone())),
                pulumi_gestalt_core::ConfigValue::Secret(s) => {
                    let output = SingleThreadedOutput::new(s, engine.clone());
                    let id = self.table.push(output)
                        .with_context(|| "Failed to store secret output in resource table")?;
                    Ok(ConfigValue::Secret(id))
                }
            }
        }).transpose()?;
        Ok(result)
    }

    fn drop(&mut self, context: Resource<Context>) -> anyhow::Result<()> {
        assert!(context.owned());
        self.table.delete(context)?;
        Ok(())
    }
}

impl types::Host for MyState {}

impl context::Host for MyState {}

impl HostOutput for MyState {
    fn map(
        &mut self,
        self_: Resource<SingleThreadedOutput>,
        function_name: String,
    ) -> anyhow::Result<Resource<SingleThreadedOutput>> {
        let table = &mut self.table;
        let output = table.get_mut(&self_)?;
        let engine = &output.engine;
        let output = &output.output_id;
        let output = engine
            .borrow_mut()
            .create_native_function_node(function_name.clone().into(), *output);

        let output = SingleThreadedOutput::new(output, engine.clone());
        let id = table.push(output)?;
        Ok(id)
    }

    fn clone(
        &mut self,
        self_: Resource<output_interface::Output>,
    ) -> anyhow::Result<Resource<output_interface::Output>> {
        assert!(!self_.owned());
        let output = self.table.get_mut(&self_)?;
        let output_id = output.output_id;
        let output = SingleThreadedOutput::new(output_id, output.engine.clone());
        let id = self.table.push(output)?;
        Ok(id)
    }

    fn combine(
        &mut self,
        self_: Resource<output_interface::Output>,
        outputs: Vec<Resource<output_interface::Output>>,
    ) -> anyhow::Result<Resource<output_interface::Output>> {
        assert!(!self_.owned());
        let table = &mut self.table;
        let st_output = table.get(&self_)?;
        let output = &st_output.output_id;

        let mut outputs2 = Vec::new();
        outputs2.push(*output);
        for field in outputs {
            let output = table.get(&field)?;
            let output = &output.output_id;
            outputs2.push(*output)
        }

        let result = st_output
            .engine
            .borrow_mut()
            .create_combine_outputs(outputs2);
        let output = SingleThreadedOutput::new(result, st_output.engine.clone());
        let id = table.push(output)?;
        Ok(id)
    }

    fn add_to_export(
        &mut self,
        self_: Resource<output_interface::Output>,
        name: String,
    ) -> anyhow::Result<()> {
        assert!(!self_.owned());
        let table = &mut self.table;
        let output = table.get_mut(&self_)?;
        let engine = &output.engine;
        let output = &output.output_id;
        engine.borrow_mut().add_output(name.clone().into(), *output);
        Ok(())
    }

    fn drop(&mut self, rep: Resource<output_interface::Output>) -> anyhow::Result<()> {
        assert!(rep.owned());
        self.table.delete(rep)?;
        Ok(())
    }
}

impl HostCompositeOutput for MyState {
    fn get_field(
        &mut self,
        self_: Resource<output_interface::CompositeOutput>,
        field_name: String,
    ) -> anyhow::Result<Resource<output_interface::Output>> {
        assert!(!self_.owned());
        let output = self.table.get_mut(&self_)?;
        let engine = &output.engine;
        let output = &output.output_id;
        let output = engine
            .borrow_mut()
            .create_extract_field(field_name.clone().into(), *output);
        let output = SingleThreadedOutput::new(output, engine.clone());
        let id = self.table.push(output)?;
        Ok(id)
    }

    fn drop(&mut self, rep: Resource<output_interface::CompositeOutput>) -> anyhow::Result<()> {
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
            .inherit_stdin()
            .inherit_stdout()
            .inherit_env()
            .build();

        let mut store = Store::new(
            &engine,
            SimplePluginCtx {
                // logger: SimpleLogger {},
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
