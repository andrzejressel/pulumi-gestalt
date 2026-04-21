/// Generates Rust source code from a PCL program.
///
/// Pipeline: PCL IR → Domain IR → Rust IR → String
use crate::domain_ir::Program;
use crate::pcl_model::PclProtobufProgram;
use crate::rust_ir::RustFile;
use rootcause::Result;
use rootcause::prelude::ResultExt;

pub struct GenerateResult {
    pub main_rs: String,
    pub domain: Program,
    pub rust_ir: RustFile,
}

pub fn generate_main(model_program: &PclProtobufProgram) -> Result<GenerateResult> {
    let domain =
        crate::pcl_to_domain::lower(model_program).context("Failed to lower PCL to domain IR")?;
    let rust_ir =
        crate::domain_to_rust::lower(&domain).context("Failed to lower domain IR to Rust IR")?;
    let main_rs =
        crate::rust_to_string::render(&rust_ir).context("Failed to render Rust IR to string")?;
    Ok(GenerateResult {
        main_rs,
        domain,
        rust_ir,
    })
}
