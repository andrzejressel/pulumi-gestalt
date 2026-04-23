mod macros;
pub use macros::ToOutput;
mod native;

#[doc(hidden)]
#[path = "private/mod.rs"]
pub mod __private;
pub mod any_export;
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
    CompositeOutput, Context, CustomResourceOptions, InvokeResourceRequest, Output, Provider,
    RegisterResourceRequest, ResourceRequestObjectField,
};

/// Entrypoint for execution
/// ```rust,no_run
/// pulumi_gestalt_rust::run(|ctx| {
///     // your code here
///     let output = ctx.new_output(&"Hello, Pulumi!");
///     pulumi_gestalt_rust::add_export("greeting", &output);
///     Ok(())
/// }).unwrap();
pub fn run<F: Fn(&Context) -> Result<()>>(f: F) -> Result<()> {
    let ctx = Context::new();
    f(&ctx).context("Failed to run Pulumi program")?;
    ctx.finish();
    Ok(())
}

/// Add the given [Output] to [Stack Output](https://www.pulumi.com/tutorials/building-with-pulumi/stack-outputs/)
pub fn add_export<T>(name: &str, output: &Output<T>) {
    output.add_to_export(name);
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
