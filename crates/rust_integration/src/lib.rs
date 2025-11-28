use anyhow::Result;
use pulumi_gestalt_schema::model::Package;

mod engine;
pub mod finish;

pub use engine::*;
pub use pulumi_gestalt_domain::FieldName;

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
