use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use pulumi_gestalt_schema::model::Package;

mod code_generation;
mod description;
mod model;
mod output;
mod utils;

pub fn generate_rust(
    package: &Package,
    result_path: &Path,
) -> Result<()> {

    fs::create_dir_all(result_path)?;

    output::generate_combined_code(&package, result_path);

    Ok(())
}