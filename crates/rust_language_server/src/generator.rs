/// Generates Rust source code from a PCL program.
///
/// Pipeline: PCL IR → Dynamic Domain IR → Typesafe Domain IR → Rust IR → String
use crate::dynamic_domain_ir::Program as DynamicProgram;
use crate::pcl_model::PclProtobufProgram;
use crate::rust_ir::RustFile;
use crate::typesafe_domain_ir::Program as TypesafeProgram;
use rootcause::Result;
use rootcause::prelude::ResultExt;

pub struct GenerateResult {
    pub main_rs: String,
    pub dynamic_domain: DynamicProgram,
    pub typesafe_domain: TypesafeProgram,
    pub rust_ir: RustFile,
}

pub fn generate_main(model_program: &PclProtobufProgram) -> Result<GenerateResult> {
    let dynamic_domain = crate::pcl_to_dynamic_domain::lower(model_program)
        .context("Failed to lower PCL to dynamic domain IR")?;
    let typesafe_domain = crate::dynamic_to_typesafe_domain::lower(&dynamic_domain)
        .context("Failed to lower dynamic domain IR to typesafe domain IR")?;
    let rust_ir = crate::typesafe_domain_to_rust::lower(&typesafe_domain)
        .context("Failed to lower typesafe domain IR to Rust IR")?;
    let packages_expr = render_packages_expr(model_program);
    let main_rs = crate::rust_to_string::render(&rust_ir, &packages_expr)
        .context("Failed to render Rust IR to string")?;
    Ok(GenerateResult {
        main_rs,
        dynamic_domain,
        typesafe_domain,
        rust_ir,
    })
}

fn render_packages_expr(model_program: &PclProtobufProgram) -> String {
    let mut plugins = model_program
        .plugins
        .iter()
        .filter(|plugin| plugin.name != "pulumi")
        .map(|plugin| plugin.name.replace("-", "_"))
        .collect::<Vec<_>>();
    plugins.sort();
    plugins.dedup();

    if plugins.is_empty() {
        return "vec![]".to_string();
    }

    let items = plugins
        .into_iter()
        .map(|name| format!("pulumi_{name}::package()"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("vec![{items}]")
}
