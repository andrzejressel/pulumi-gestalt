use crate::schema::Package;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[cfg(feature = "arbitrary")]
mod arbitrary;
mod filter;
pub mod model;
mod schema;
mod utils;

/// Requires `pulumi` CLI to be installed and available in PATH
pub fn get_schema(
    provider_name: &str,
    provider_version: &str,
    modules: Option<&[&str]>,
) -> Result<model::Package> {
    let schema_output = std::process::Command::new("pulumi")
        .arg("package")
        .arg("get-schema")
        .arg(format!("{}@{}", provider_name, provider_version))
        .env("PULUMI_AWS_MINIMAL_SCHEMA", "true") // https://github.com/pulumi/pulumi-aws/issues/2565
        .output()
        .context("Failed to execute pulumi command")?;

    let schema =
        String::from_utf8(schema_output.stdout).context("Invalid UTF-8 in pulumi output")?;

    let package =
        deserialize_package_json(&schema, modules).context("Failed to deserialize package")?;

    Ok(package)
}

// Supports both JSON and YAML files
pub fn deserialize_package_file(
    schema_file: &Path,
    filter: Option<&[&str]>,
) -> Result<model::Package> {
    let schema_package: Package = extract_schema_from_file(schema_file)?;
    let mut package = schema::to_model(&schema_package)?;
    if let Some(filter) = filter {
        filter::filter_package(&mut package, filter);
    }

    Ok(package)
}

fn deserialize_package_json(
    schema_content: &str,
    filter: Option<&[&str]>,
) -> Result<model::Package> {
    let schema_package: Package =
        serde_json::from_str(schema_content).map_err(anyhow::Error::new)?;
    let mut package = schema::to_model(&schema_package)?;
    if let Some(filter) = filter {
        filter::filter_package(&mut package, filter);
    }

    Ok(package)
}

fn extract_schema_from_file(schema_json: &Path) -> Result<Package> {
    let file = File::open(schema_json)
        .with_context(|| format!("Error opening schema file [{:?}]", schema_json))?;
    let reader = BufReader::new(file);
    match schema_json.extension().and_then(|s| s.to_str()) {
        None => Err(anyhow::anyhow!(
            "No extensions for schema file: {}.",
            schema_json.display()
        )),
        Some("yml" | "yaml") => serde_yaml::from_reader(reader).map_err(anyhow::Error::new),
        Some("json") => serde_json::from_reader(reader).map_err(anyhow::Error::new),
        Some(ext) => Err(anyhow::anyhow!(
            "Unsupported schema file extension: {}. Only 'json', 'yaml', and 'yml' are supported.",
            ext
        )),
    }
}
