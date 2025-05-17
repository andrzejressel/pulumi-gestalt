mod source_code_resource_code;

use crate::output::resources::source_code_resource_code::generate_single_resource_source_code;
use crate::utils::reformat_code;
use anyhow::Context;
use pulumi_gestalt_schema::model::{ElementId, Package};

pub(crate) fn generate_single_file(package: &Package, element_id: &ElementId) -> String {
    reformat_code(&generate_single_resource_source_code(package, element_id))
        .context("Failed to reformat resource source code")
        .unwrap()
}
