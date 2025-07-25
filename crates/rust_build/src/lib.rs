//! `pulumi_gestalt_build` generates glue code for given Pulumi provider
//! ## Example
//! Select Pulumi provider you want to use. For demonstration purposes I'll choose [Random](https://www.pulumi.com/registry/packages/random/)
//!
//! Here we will have the provider glue and our code in single library. Due to compilation times
//! it is recommended to separate glue code and custom code.
//! ```bash
//! $ cargo new --lib random && cd random
//! ```
//!
//! First, add `pulumi_gestalt_build`, `bon`, `serde`, `anyhow` and `wit-bindgen` to `Cargo.toml`:
//!
//! ```bash
//! $ cargo add --build pulumi_gestalt_build
//! $ cargo add bon
//! $ cargo add serde --features derive
//! $ cargo add anyhow
//! $ cargo add wit-bindgen
//! ```
//!
//! To generate glue code, use `pulumi_gestalt_build` in `build.rs`
//! ```rust,no_run
//! use std::error::Error;
//! fn main() -> Result<(), Box<dyn Error>> {
//!     pulumi_gestalt_build::generate("random", "4.15.0")?;
//!     Ok(())
//! }
//! ```
//!
//! Then add generated code to `lib.rs` and use it in `#[pulumi_main]`
//! ```rust,ignore
//! mod random {
//!   include_provider!("random");
//! }
//!
//! use random::random_string::RandomStringArgs;
//! use random::random_string;
//! use pulumi_gestalt_rust::pulumi_main;
//! use anyhow::Result;
//!
//! #[pulumi_main]
//! fn main() -> Result<()> {
//!   let random_string = random_string::create(
//!     "test",
//!     RandomStringArgs::builder().length(32).build_struct()
//!   );
//!   Ok(())
//! }
//! ```

use anyhow::{Context, Result};
use pulumi_gestalt_generator::generate_rust;
use pulumi_gestalt_schema::{deserialize_package_file, get_schema};
use std::env;
use std::path::Path;

/// Generates glue code for given provider, version and modules. Can be included using [`pulumi_gestalt_rust::include_provider!(provider_name)`]
/// Modules for provider can be found in Pulumi registry on left side with (M) icon:
/// - [AWS](https://www.pulumi.com/registry/packages/aws/)
/// - [Azure](https://www.pulumi.com/registry/packages/azure/)
/// - [GCP](https://www.pulumi.com/registry/packages/gcp/)
pub fn generate_with_filter(
    provider_name: &str,
    provider_version: &str,
    modules: &[&str],
) -> Result<()> {
    generate_with_optional_filter(provider_name, provider_version, Some(modules))
}

/// Generates glue code for given provider and version. Can be included using [`pulumi_gestalt_rust::include_provider!(provider_name)`]
pub fn generate(provider_name: &str, provider_version: &str) -> Result<()> {
    generate_with_optional_filter(provider_name, provider_version, None)
}

/// Generates glue code for given schema json/yaml. Can be included using [`pulumi_gestalt_rust::include_provider!(provider_name)`]
pub fn generate_from_schema(schema_file: &Path) -> Result<()> {
    generate_from_schema_with_optional_filter(schema_file, None)
}

/// Generates glue code for given schema json/yaml and modules. Can be included using [`pulumi_gestalt_rust::include_provider!(provider_name)`]
/// Modules for provider can be found in Pulumi registry on left side with (M) icon:
/// - [AWS](https://www.pulumi.com/registry/packages/aws/)
/// - [Azure](https://www.pulumi.com/registry/packages/azure/)
/// - [GCP](https://www.pulumi.com/registry/packages/gcp/)
pub fn generate_from_schema_with_filter(schema_file: &Path, modules: &[&str]) -> Result<()> {
    generate_from_schema_with_optional_filter(schema_file, Some(modules))
}

fn generate_from_schema_with_optional_filter(
    schema_file: &Path,
    modules: Option<&[&str]>,
) -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").context("Failed to get OUT_DIR environment variable")?;
    let out_dir = out_dir
        .to_str()
        .context(format!("Failed to convert [{:?}] to string", out_dir))?;

    let package =
        deserialize_package_file(schema_file, modules).context("Failed to deserialize package")?;

    let location = Path::new(out_dir).join("pulumi").join(&package.name);
    generate_rust(&package, &location).context("Failed to generate glue files")?;

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={}", schema_file.display());
    Ok(())
}

fn generate_with_optional_filter(
    provider_name: &str,
    provider_version: &str,
    modules: Option<&[&str]>,
) -> Result<()> {
    let package =
        get_schema(provider_name, provider_version, modules).context("Failed to get schema")?;

    let out_dir = env::var_os("OUT_DIR").context("Failed to get OUT_DIR environment variable")?;
    let out_dir = out_dir
        .to_str()
        .context(format!("Failed to convert [{:?}] to string", out_dir))?;

    let location = Path::new(out_dir).join("pulumi").join(provider_name);
    generate_rust(&package, &location).context("Failed to generate glue files")?;

    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}
