use crate::model::{InputPropertyExt, OutputPropertyExt, TypeExt};
use handlebars::Handlebars;
use pulumi_gestalt_schema::model::{Package, Type};
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("provider_code.rs.handlebars");

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
struct Provider {
    name: String,
    r#type: String,
    package_name: String,
    input_properties: Vec<InputProperty>,
    output_properties: Vec<OutputProperty>,
    struct_name: String,
    function_name: String,
    description_lines: Vec<String>,
    get_version: String,
}

fn convert_provider(package: &Package) -> Provider {
    let provider = &package.provider;
    let depth = 1; // It will be inside 'pub mod provider' WHICH IS in main.rs
    let get_version = "super::get_version()".to_string();
    Provider {
        name: "provider".to_string(),
        r#type: format!("pulumi:providers:{}", package.name),
        package_name: "provider".to_string(),
        struct_name: "Provider".to_string(),
        get_version,
        function_name: "create".to_string(),
        description_lines: crate::utils::to_lines(provider.description.clone(), package),
        input_properties: provider
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
        output_properties: provider
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

pub(crate) fn generate_provider_source_code(package: &Package) -> String {
    let handlebars = Handlebars::new();
    let provider = convert_provider(package);
    handlebars
        .render_template(TEMPLATE, &json!({"resource": provider}))
        .unwrap()
}
