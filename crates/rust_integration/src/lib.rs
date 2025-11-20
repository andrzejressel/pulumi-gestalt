use std::sync::{Arc, Mutex};
use pulumi_gestalt_schema::model::Package;
use anyhow::{Context as anyhowContext, Result};
use pulumi_gestalt_core::{Config, Engine};
use pulumi_gestalt_grpc_connection::RealPulumiConnector;

mod engine;
pub use engine::*;


pub(crate) async fn get_engine<FunctionContext>() -> engine::Context<FunctionContext> {
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

    engine::Context {
        inner: Arc::new(Mutex::new(Engine::new(pulumi_connector, config)))
    }
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
) -> Result<Package> {
    pulumi_gestalt_schema::get_schema(provider_name, provider_version, modules)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
