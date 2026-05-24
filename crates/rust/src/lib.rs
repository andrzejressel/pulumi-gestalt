mod macros;
pub use macros::ToOutput;
mod native;

#[doc(hidden)]
#[path = "private/mod.rs"]
pub mod __private;
mod input;
mod oneof;
mod pulumi_any;
pub mod resources;
pub mod stdlib;

pub use input::Input;
pub use pulumi_any::{PulumiAny, ToPulumiAny};

pub use oneof::OneOf2;
pub use oneof::OneOf3;
pub use oneof::OneOf4;

use anyhow::{Context as AnyhowContext, Result};
pub use native::{
    CompositeOutput, Context, CustomResourceOptions, InvokeResourceRequest, Provider,
    RegisterResourceRequest, ResourceRequestObjectField,
};
pub use pulumi_gestalt_model::FromPulumiValue;
pub use pulumi_gestalt_model::Output;
pub use pulumi_gestalt_model::ToPulumiValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageParameterization {
    pub name: String,
    pub version: String,
    pub value: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub kind: String,
    pub version: String,
    pub server: String,
    pub checksums: BTreeMap<String, Vec<u8>>,
    pub parameterization: Option<PackageParameterization>,
}

/// Entrypoint for execution
/// ```rust,no_run
/// pulumi_gestalt_rust::run(|ctx| {
///     // your code here
///     let output = ctx.new_output(&"Hello, Pulumi!".to_string());
///     ctx.add_export("greeting", &output);
///     Ok(())
/// }).unwrap();
pub fn run<F: Fn(&Context) -> Result<()>>(f: F) -> Result<()> {
    run_with_packages(vec![], f)
}

pub fn run_with_packages<F: Fn(&Context) -> Result<()>>(
    packages: Vec<Package>,
    f: F,
) -> Result<()> {
    if try_write_packages_from_args(&packages)? {
        return Ok(());
    }

    let ctx = Context::new();
    f(&ctx).context("Failed to run Pulumi program")?;
    ctx.finish();
    Ok(())
}

fn try_write_packages_from_args(packages: &[Package]) -> Result<bool> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() >= 3 && args[1] == "get-packages" {
        let output_path = &args[2];
        let serialized =
            serde_json::to_string(packages).context("Failed to serialize packages metadata")?;
        std::fs::write(output_path, serialized)
            .with_context(|| format!("Failed to write packages file at [{output_path}]"))?;
        return Ok(true);
    }

    Ok(false)
}

/// Load specific generated provider
///
/// build.rs:
/// ```rust,no_run
/// use std::error::Error;
/// fn main() -> Result<(), Box<dyn Error>> {
///     pulumi_gestalt_build::generate("random", "4.15.0")?;
///     Ok(())
/// }
/// ```
///
/// lib.rs
/// ```rust,ignore
/// include_provider!("random");
/// ```
#[macro_export]
macro_rules! include_provider {
    ($file:expr) => {
        include!(concat!(env!("OUT_DIR"), "/pulumi/", $file, "/lib.rs"));
    };
}
