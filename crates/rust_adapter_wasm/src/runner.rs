use crate::WasmContext;
use anyhow::{Context, Error, Result};
use log::{error, info};
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::types::FunctionInvocationRequest;
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::types::FunctionInvocationResult;

pub fn run<F>(f: F) -> Result<(), Error>
where
    F: Fn(&WasmContext) -> Result<(), Error>,
{
    let context = WasmContext::new();
    f(&context)
        .context("Failed to run main function")?;
    run_loop(&context)
        .context("Failed to run loop")?;
    Ok(())
}

fn run_loop(context: &WasmContext) -> Result<(), Error> {
    run_all_function(context)
}

fn run_all_function(context: &WasmContext) -> Result<(), Error> {
    let mut functions = context.invoke_finish(vec![])
        .context("Failed to run 'invoke_finish'")?;

    loop {
        if functions.is_empty() {
            return Ok(());
        }
        let mapped = map_functions(context, &functions)
            .context("Failed to run `map_functions`")?;
        functions = context.invoke_finish(mapped)?;
    }
}

fn map_functions<'a>(
    context: &'a WasmContext,
    functions: &'a [FunctionInvocationRequest],
) -> Result<Vec<FunctionInvocationResult<'a>>> {
    functions
        .iter()
        .map(
            |FunctionInvocationRequest {
                 id,
                 function_name,
                 value,
             }| {
                info!("Invoking function [{function_name}] with value [{value:?}]");
                let result = context
                    .invoke_function(function_name, value)
                    .with_context(|| format!("Error invoking function [{function_name}]"))?;
                Ok(FunctionInvocationResult { id, value: result })
            },
        )
        .collect()
}
