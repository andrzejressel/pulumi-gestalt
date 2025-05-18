use crate::model::{ElementIdExt, InputPropertyExt, OutputPropertyExt, TypeExt};
use crate::output::get_register_interface;
use crate::utils::access_root;
use handlebars::Handlebars;
use pulumi_gestalt_schema::model::{ElementId, Package, Type};
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("resource_code.rs.handlebars");

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
struct Resource {
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

fn convert_resource(package: &Package, element_id: &ElementId) -> Resource {
    let resource = package.resources.get(element_id).unwrap();
    let depth = element_id.namespace.len() + 1;
    let get_version = format!("{}get_version()", access_root(depth));
    Resource {
        name: element_id.get_rust_namespace_name(),
        r#type: element_id.raw.clone(),
        package_name: element_id.get_rust_package_name(),
        register_interface: get_register_interface(element_id),
        struct_name: element_id.name.clone(),
        get_version,
        function_name: element_id.get_rust_function_name(),
        description_lines: crate::utils::to_lines(
            resource.description.clone(),
            package,
            Some(element_id.clone()),
        ),
        input_properties: resource
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
                    None,
                ),
            })
            .collect(),
        output_properties: resource
            .output_properties
            .iter()
            .map(|output_property| OutputProperty {
                name: output_property.name.clone(),
                arg_name: output_property.get_rust_argument_name(),
                type_: output_property.r#type.get_rust_type(depth),
                description_lines: crate::utils::to_lines(
                    output_property.description.clone(),
                    package,
                    None,
                ),
            })
            .collect(),
    }
}

pub(crate) fn generate_single_resource_source_code(
    package: &Package,
    element_id: &ElementId,
) -> String {
    let handlebars = Handlebars::new();
    let resource = convert_resource(package, element_id);
    handlebars
        .render_template(TEMPLATE, &json!({"resource": resource}))
        .unwrap()
}
