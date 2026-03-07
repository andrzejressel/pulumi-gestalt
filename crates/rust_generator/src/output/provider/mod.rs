mod source_code_provider_code;

use crate::output::provider::source_code_provider_code::generate_provider_source_code;
use crate::utils::reformat_code;
use anyhow::Context;
use pulumi_gestalt_schema::model::Package;

pub(crate) fn generate_code(package: &Package) -> String {
    reformat_code(&generate_provider_source_code(package))
        .context("Failed to reformat provider source code")
        .unwrap()
}
