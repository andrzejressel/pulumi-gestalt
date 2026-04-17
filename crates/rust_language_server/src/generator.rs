/// Generates Rust source code from a PCL program.
///
/// Pipeline: PCL IR → Domain IR → Rust IR → String
use crate::pcl_model::PclProtobufProgram;
use rootcause::Result;
use rootcause::prelude::ResultExt;

pub fn generate_main(model_program: &PclProtobufProgram) -> Result<String> {
    let domain =
        crate::pcl_to_domain::lower(model_program).context("Failed to lower PCL to domain IR")?;
    let rust = crate::domain_to_rust::lower(&domain);
    Ok(crate::rust_to_string::render(&rust).context("Failed to render Rust IR to string")?)
}
