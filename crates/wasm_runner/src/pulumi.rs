use std::sync::Arc;
use crate::pulumi::runner::Client;
use anyhow::Error;
use log::info;
use prost::Message;
use pulumi_gestalt_grpc_connection::pulumi_state::PulumiState;
use pulumi_gestalt_proto::mini::pulumirpc::{
    RegisterResourceOutputsRequest, RegisterResourceRequest, RegisterResourceResponse,
    ResourceInvokeRequest,
};
use pulumi_gestalt_wit::bindings_runner as runner;
use wasmtime::Store;
use wasmtime::component::{Component, Linker, Resource, ResourceTable};
use wasmtime_wasi::{IoView, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi::bindings::sync::io::streams::HostInputStream;
use pulumi_gestalt_rust_integration::ObjectField;
use pulumi_gestalt_wit::bindings_runner::component::pulumi_gestalt::{context, output_interface, types};
use pulumi_gestalt_wit::bindings_runner::component::pulumi_gestalt::context::{CompositeOutput, ConfigValue, Context, FunctionInvocationRequest, FunctionInvocationResult, HostContext, Output};
use pulumi_gestalt_wit::bindings_runner::component::pulumi_gestalt::output_interface::{HostCompositeOutput, HostOutput};

pub struct Pulumi {
    plugin: Client,
    store: Store<SimplePluginCtx>,
}

struct SimplePluginCtx {
    table: ResourceTable,
    context: WasiCtx,
    my_state: MyState,
}

struct MyState {
    pulumi_state: PulumiState,
    in_preview: bool,
    table: ResourceTable
}

impl HostContext for MyState {
    fn new(&mut self) -> anyhow::Result<Resource<Context>> {
        let context = Context::create_context();
        let id = self.table.push(context)?;
        Ok(id)
    }

    fn create_output(&mut self, context: Resource<Context>, value: String, secret: bool) -> anyhow::Result<Resource<Output>> {
        assert!(!context.owned());
        let context = self.table.get_mut(&context)?;
        let output = context.create_output(value, secret);
        let id = self.table.push(output)?;
        Ok(id)
    }

    fn register_resource(&mut self, context: Resource<Context>, request: context::RegisterResourceRequest) -> anyhow::Result<Resource<CompositeOutput>> {
        assert!(!context.owned());
        let mut table = &mut self.table;
        
        let mut inputs = Vec::new();
        for field in request.object {
            let output = table.get(&field.value)?;
            inputs.push(ObjectField {
                name: field.name,
                value: output
            })
        }

        let context = table.get(&context)?;

        let result = context.register_resource(pulumi_gestalt_rust_integration::RegisterResourceRequest {
            type_: request.type_,
            name: request.name,
            version: request.version,
            inputs: &inputs,
        });
        
        let id = self.table.push(result)?;
        Ok(id)
    }

    fn invoke_resource(&mut self, context: Resource<Context>, request: context::ResourceInvokeRequest) -> anyhow::Result<Resource<CompositeOutput>> {
        assert!(!context.owned());
        todo!()
    }

    fn finish(&mut self, context: Resource<Context>, functions: Vec<FunctionInvocationResult>) -> anyhow::Result<Vec<FunctionInvocationRequest>> {
        assert!(!context.owned());
        todo!()
    }

    fn get_config(&mut self, context: Resource<Context>, name: Option<String>, key: String) -> anyhow::Result<Option<ConfigValue>> {
        assert!(!context.owned());
        todo!()
    }

    fn drop(&mut self, context: Resource<Context>) -> anyhow::Result<()> {
        assert!(context.owned());
        self.table.delete(context)?;
        Ok(())
    }
}

impl types::Host for MyState {
    
}

impl context::Host for MyState {
    
}

impl HostOutput for MyState {
    fn map(&mut self, self_: Resource<output_interface::Output>, function_name: String) -> anyhow::Result<Resource<output_interface::Output>> {
        todo!()
    }

    fn clone(&mut self, self_: Resource<output_interface::Output>) -> anyhow::Result<Resource<output_interface::Output>> {
        todo!()
    }

    fn combine(&mut self, self_: Resource<output_interface::Output>, outputs: Vec<Resource<output_interface::Output>>) -> anyhow::Result<Resource<output_interface::Output>> {
        todo!()
    }

    fn add_to_export(&mut self, self_: Resource<output_interface::Output>, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn drop(&mut self, rep: Resource<output_interface::Output>) -> anyhow::Result<()> {
        todo!()
    }
}

impl HostCompositeOutput for MyState {
    fn get_field(&mut self, self_: Resource<output_interface::CompositeOutput>, field_name: String) -> anyhow::Result<Resource<output_interface::Output>> {
        todo!()
    }

    fn drop(&mut self, rep: Resource<output_interface::CompositeOutput>) -> anyhow::Result<()> {
        todo!()
    }
}

impl output_interface::Host for MyState {
    
}

impl IoView for SimplePluginCtx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for SimplePluginCtx {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.context
    }
}

impl Pulumi {
    pub async fn create(
        pulumi_gestalt_file: Vec<u8>,
        pulumi_monitor_url: String,
        pulumi_engine_url: String,
        pulumi_stack: String,
        pulumi_project: String,
        in_preview: bool,
    ) -> Result<Pulumi, Error> {
        let mut engine_config = wasmtime::Config::new();
        engine_config.wasm_component_model(true);
        engine_config.async_support(true);
        engine_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        engine_config.debug_info(true);

        let engine = wasmtime::Engine::new(&engine_config)?;

        let mut linker: Linker<SimplePluginCtx> = Linker::new(&engine);
        Client::add_to_linker(&mut linker, |state: &mut SimplePluginCtx| {
            &mut state.my_state
        })?;

        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        let table = ResourceTable::new();
        let table2 = ResourceTable::new();

        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdin()
            .inherit_stdout()
            .inherit_env()
            .build();

        let pulumi_state = PulumiState::new(
            pulumi_monitor_url.clone(),
            pulumi_engine_url.clone(),
            pulumi_project.clone(),
            pulumi_stack.clone(),
        )
        .await?;

        let mut store = Store::new(
            &engine,
            SimplePluginCtx {
                // logger: SimpleLogger {},
                table,
                context: wasi_ctx,
                my_state: MyState {
                    pulumi_state,
                    in_preview,
                    table: table2,
                },
            },
        );

        info!("Creating Wasm component");
        let component = Component::from_binary(&engine, &pulumi_gestalt_file)?;
        info!("Instantiating Wasm component");
        let plugin = Client::instantiate(&mut store, &component, &linker)?;
        info!("Wasm component instantiated");

        Ok(Pulumi { plugin, store })
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        self.plugin
            .component_pulumi_gestalt_external_pulumi_main()
            .call_main(&mut self.store)?;

        Ok(())
    }
}
