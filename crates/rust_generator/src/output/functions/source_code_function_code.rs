use crate::model::{ElementIdExt, InputPropertyExt, OutputPropertyExt, TypeExt};
use crate::output::get_register_interface;
use crate::utils::access_root;
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use pulumi_gestalt_schema::model::{ElementId, Package, Type};
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("function_code.rs.handlebars");

#[derive(Serialize)]
struct InputProperty {
    name: String,
    arg_name: String,
    type_: String,
    description_lines: Vec<String>,
    default: bool,
    skip: bool,
    private: bool,
}

#[derive(Serialize)]
struct OutputProperty {
    name: String,
    arg_name: String,
    type_: String,
    description_lines: Vec<String>,
}

#[derive(Serialize)]
struct Function {
    name: String,
    r#type: String,
    package_name: String,
    input_properties: Vec<InputProperty>,
    output_properties: Vec<OutputProperty>,
    struct_name: String,
    function_name: String,
    description_lines: Vec<String>,
    register_interface: String,
    get_version: String,
}

fn convert_function(package: &Package, element_id: &ElementId) -> Function {
    let function = package.functions.get(element_id).unwrap();
    let depth = element_id.namespace.len() + 2;
    let get_version = format!("{}get_version()", access_root(depth));
    Function {
        name: element_id.get_rust_namespace_name(),
        r#type: element_id.raw.clone(),
        package_name: element_id.get_rust_package_name(),
        struct_name: element_id.name.clone().to_case(Case::Pascal),
        register_interface: get_register_interface(element_id),
        function_name: element_id.get_rust_function_name(),
        get_version,
        description_lines: crate::utils::to_lines(function.description.clone(), package),
        input_properties: function
            .input_properties
            .iter()
            .map(|input_property| InputProperty {
                name: input_property.name.clone(),
                arg_name: input_property.get_rust_argument_name(),
                default: matches!(input_property.r#type, Type::Option(_)),
                skip: matches!(input_property.r#type, Type::ConstString(_)),
                private: matches!(input_property.r#type, Type::ConstString(_)),
                type_: input_property.r#type.get_rust_type(depth),
                description_lines: crate::utils::to_lines(
                    input_property.description.clone(),
                    package,
                ),
            })
            .collect(),
        output_properties: function
            .output_properties
            .iter()
            .map(|output_property| OutputProperty {
                name: output_property.name.clone(),
                arg_name: output_property.get_rust_argument_name(),
                type_: output_property.r#type.get_rust_type(depth),
                description_lines: crate::utils::to_lines(
                    output_property.description.clone(),
                    package,
                ),
            })
            .collect(),
    }
}

pub(crate) fn generate_single_function_source_code(
    package: &Package,
    element_id: &ElementId,
) -> String {
    let handlebars = Handlebars::new();
    let function = convert_function(package, element_id);

    handlebars
        .render_template(TEMPLATE, &json!({"function": function}))
        .unwrap()
}
