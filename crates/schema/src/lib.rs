use crate::schema::Package;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod converter;
mod filter;
pub mod model;
mod schema;
mod utils;

pub fn deserialize_package(schema_json: &Path, filter: Option<&[&str]>) -> Result<model::Package> {
    let schema_package: schema::Package = extract_schema_from_file(schema_json)?;
    let mut package = schema::to_model(&schema_package)?;
    if let Some(filter) = filter {
        filter::filter_package(&mut package, filter);
    }

    Ok(package)
}

fn extract_schema_from_file(schema_json: &Path) -> anyhow::Result<Package> {
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
            "Unsupported schema file extension: {}.",
            ext
        )),
    }
}

pub fn extract_micro_package(schema_json: &Path) -> anyhow::Result<MicroPackage> {
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
            "Unsupported schema file extension: {}.",
            ext
        )),
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct MicroPackage {
    pub name: String,
}
