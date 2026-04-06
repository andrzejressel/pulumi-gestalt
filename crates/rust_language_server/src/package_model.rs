use pulumi_gestalt_proto::language_server::pulumipackage as pb;
use pulumi_gestalt_schema::model;
use std::collections::BTreeMap;

pub fn map_package(package: pb::Package) -> model::Package {
    let resources = package
        .resources
        .into_iter()
        .map(map_resource)
        .collect::<BTreeMap<_, _>>();

    let functions = package
        .functions
        .into_iter()
        .map(map_function)
        .collect::<BTreeMap<_, _>>();

    let types = package
        .types
        .into_iter()
        .map(map_global_type)
        .collect::<BTreeMap<_, _>>();

    model::Package::new(
        package.name,
        non_empty(package.display_name),
        non_empty(package.plugin_download_url),
        package.version,
        map_provider(package.provider),
        resources,
        functions,
        types,
    )
}

fn map_provider(provider: Option<pb::Provider>) -> model::Provider {
    let Some(provider) = provider else {
        return model::Provider::default();
    };

    model::Provider {
        description: non_empty(provider.description),
        input_properties: provider
            .input_properties
            .into_iter()
            .map(map_input_property)
            .collect(),
        output_properties: provider
            .output_properties
            .into_iter()
            .map(map_output_property)
            .collect(),
    }
}

fn map_resource(resource: pb::Resource) -> (model::ElementId, model::Resource) {
    let element_id = map_element_id(
        resource
            .element_id
            .expect("resource.element_id is required"),
    );

    (
        element_id.clone(),
        model::Resource {
            element_id: element_id.clone(),
            description: non_empty(resource.description),
            input_properties: resource
                .input_properties
                .into_iter()
                .map(map_input_property)
                .collect(),
            output_properties: resource
                .output_properties
                .into_iter()
                .map(map_output_property)
                .collect(),
        },
    )
}

fn map_function(function: pb::Function) -> (model::ElementId, model::Function) {
    let element_id = map_element_id(
        function
            .element_id
            .expect("function.element_id is required"),
    );

    let _ = function.return_type;

    (
        element_id.clone(),
        model::Function {
            element_id: element_id.clone(),
            description: non_empty(function.description),
            input_properties: function
                .input_properties
                .into_iter()
                .map(map_input_property)
                .collect(),
            output_properties: function
                .output_properties
                .into_iter()
                .map(map_output_property)
                .collect(),
        },
    )
}

fn map_global_type(global_type: pb::GlobalType) -> (model::ElementId, model::GlobalType) {
    let element_id = map_element_id(
        global_type
            .element_id
            .expect("global_type.element_id is required"),
    );

    (
        element_id.clone(),
        model::GlobalType {
            element_id,
            value: map_global_type_value(
                global_type
                    .global_type_value
                    .expect("global_type.global_type_value is required"),
            ),
        },
    )
}

fn map_global_type_value(value: pb::GlobalTypeValue) -> model::GlobalTypeValue {
    match value.value.expect("global_type_value.value is required") {
        pb::global_type_value::Value::Object(object) => model::GlobalTypeValue::Object(
            non_empty(object.description),
            object
                .properties
                .into_iter()
                .map(map_global_type_property)
                .collect(),
        ),
        pb::global_type_value::Value::StringEnum(enum_type) => model::GlobalTypeValue::StringEnum(
            non_empty(enum_type.description),
            enum_type
                .elements
                .into_iter()
                .map(|element| model::StringEnumElement {
                    name: element.name,
                    value: element.value,
                    description: non_empty(element.description),
                })
                .collect(),
        ),
        pb::global_type_value::Value::NumberEnum(enum_type) => model::GlobalTypeValue::NumberEnum(
            non_empty(enum_type.description),
            enum_type
                .elements
                .into_iter()
                .map(|element| model::NumberEnumElement {
                    name: element.name,
                    value: element.value,
                    description: non_empty(element.description),
                })
                .collect(),
        ),
        pb::global_type_value::Value::IntegerEnum(enum_type) => {
            model::GlobalTypeValue::IntegerEnum(
                non_empty(enum_type.description),
                enum_type
                    .elements
                    .into_iter()
                    .map(|element| model::IntegerEnumElement {
                        name: element.name,
                        value: element.value,
                        description: non_empty(element.description),
                    })
                    .collect(),
            )
        }
    }
}

fn map_global_type_property(property: pb::GlobalTypeProperty) -> model::GlobalTypeProperty {
    model::GlobalTypeProperty {
        name: property.name,
        r#type: map_type(
            property
                .r#type
                .expect("global_type_property.type is required"),
        ),
        description: non_empty(property.description),
    }
}

fn map_input_property(property: pb::InputProperty) -> model::InputProperty {
    model::InputProperty {
        name: property.name,
        r#type: map_type(property.r#type.expect("input_property.type is required")),
        description: non_empty(property.description),
    }
}

fn map_output_property(property: pb::OutputProperty) -> model::OutputProperty {
    model::OutputProperty {
        name: property.name,
        r#type: map_type(property.r#type.expect("output_property.type is required")),
        description: non_empty(property.description),
    }
}

fn map_type(r#type: pb::Type) -> model::Type {
    match r#type.type_value.expect("type.type_value is required") {
        pb::r#type::TypeValue::BoolType(_) => model::Type::Boolean,
        pb::r#type::TypeValue::IntType(_) => model::Type::Integer,
        pb::r#type::TypeValue::NumberType(_) => model::Type::Number,
        pb::r#type::TypeValue::StringType(_) => model::Type::String,
        pb::r#type::TypeValue::ArrayType(inner) => model::Type::Array(Box::new(map_type(*inner))),
        pb::r#type::TypeValue::MapType(inner) => model::Type::Object(Box::new(map_type(*inner))),
        pb::r#type::TypeValue::RefType(ref_type) => model::Type::Ref(map_ref(ref_type)),
        pb::r#type::TypeValue::OptionalType(inner) => {
            model::Type::Option(Box::new(map_type(*inner)))
        }
        pb::r#type::TypeValue::UnionType(union) => {
            // default_type/discriminator/mapping are not represented in the target model.
            model::Type::DiscriminatedUnion(union.types.into_iter().map(map_type).collect())
        }
        pb::r#type::TypeValue::ConstString(const_string) => model::Type::ConstString(const_string),
    }
}

fn map_ref(ref_type: pb::RefType) -> model::Ref {
    match ref_type.ref_value.expect("ref_type.ref_value is required") {
        pb::ref_type::RefValue::TypeRef(element_id) => model::Ref::Type(map_element_id(element_id)),
        pb::ref_type::RefValue::Archive(_) => model::Ref::Archive,
        pb::ref_type::RefValue::Asset(_) => model::Ref::Asset,
        pb::ref_type::RefValue::Any(_) => model::Ref::Any,
        pb::ref_type::RefValue::CurrentProvider(_) => model::Ref::CurrentProvider,
    }
}

fn map_element_id(element_id: pb::ElementId) -> model::ElementId {
    model::ElementId {
        namespace: element_id.namespace,
        name: element_id.name,
        raw: element_id.raw,
    }
}

fn non_empty(value: String) -> Option<String> {
    if value.is_empty() { None } else { Some(value) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn string_type() -> pb::Type {
        pb::Type {
            type_value: Some(pb::r#type::TypeValue::StringType(pb::Empty {})),
        }
    }

    fn element_id() -> pb::ElementId {
        pb::ElementId {
            namespace: vec!["mod".to_string()],
            name: "Thing".to_string(),
            raw: "pkg:mod:Thing".to_string(),
        }
    }

    #[test]
    fn maps_package_and_normalizes_empty_optional_strings() {
        let package = pb::Package {
            name: "pkg".to_string(),
            display_name: "".to_string(),
            plugin_download_url: "".to_string(),
            version: "".to_string(),
            resources: vec![pb::Resource {
                element_id: Some(element_id()),
                description: "".to_string(),
                input_properties: vec![pb::InputProperty {
                    name: "input".to_string(),
                    r#type: Some(string_type()),
                    description: "".to_string(),
                }],
                output_properties: vec![pb::OutputProperty {
                    name: "output".to_string(),
                    r#type: Some(string_type()),
                    description: "".to_string(),
                }],
            }],
            functions: vec![pb::Function {
                element_id: Some(pb::ElementId {
                    namespace: vec![],
                    name: "Fn".to_string(),
                    raw: "pkg:index:Fn".to_string(),
                }),
                description: "".to_string(),
                input_properties: vec![],
                output_properties: vec![],
                return_type: Some(pb::Type {
                    type_value: Some(pb::r#type::TypeValue::NumberType(pb::Empty {})),
                }),
            }],
            types: vec![pb::GlobalType {
                element_id: Some(pb::ElementId {
                    namespace: vec![],
                    name: "MyType".to_string(),
                    raw: "pkg:index:MyType".to_string(),
                }),
                global_type_value: Some(pb::GlobalTypeValue {
                    value: Some(pb::global_type_value::Value::Object(pb::ObjectType {
                        description: "".to_string(),
                        properties: vec![pb::GlobalTypeProperty {
                            name: "x".to_string(),
                            r#type: Some(string_type()),
                            description: "".to_string(),
                        }],
                    })),
                }),
            }],
            provider: None,
        };

        let model = map_package(package);

        assert_eq!(model.display_name, None);
        assert_eq!(model.plugin_download_url, None);
        assert_eq!(model.version, "");
        assert_eq!(model.provider, model::Provider::default());

        let resource = model.resources.values().next().expect("missing resource");
        assert_eq!(resource.description, None);
        assert_eq!(resource.input_properties[0].description, None);
        assert_eq!(resource.output_properties[0].description, None);

        let function = model.functions.values().next().expect("missing function");
        assert_eq!(function.description, None);
    }

    #[test]
    fn maps_union_type_and_ignores_union_metadata() {
        let union = pb::Type {
            type_value: Some(pb::r#type::TypeValue::UnionType(Box::new(
                pb::DiscriminatedUnion {
                    types: vec![pb::Type {
                        type_value: Some(pb::r#type::TypeValue::StringType(pb::Empty {})),
                    }],
                    default_type: Some(Box::new(pb::Type {
                        type_value: Some(pb::r#type::TypeValue::NumberType(pb::Empty {})),
                    })),
                    discriminator: "kind".to_string(),
                    mapping: [("a".to_string(), "b".to_string())].into_iter().collect(),
                },
            ))),
        };

        let mapped = map_type(union);

        match mapped {
            model::Type::DiscriminatedUnion(types) => {
                assert_eq!(types.len(), 1);
                assert_eq!(types[0], model::Type::String);
            }
            other => panic!("unexpected type: {other:?}"),
        }
    }

    #[test]
    fn provider_none_maps_to_default() {
        let provider = map_provider(None);
        assert_eq!(provider, model::Provider::default());
    }

    #[test]
    #[should_panic(expected = "resource.element_id is required")]
    fn panics_when_required_nested_field_is_missing() {
        let _ = map_resource(pb::Resource {
            element_id: None,
            description: "x".to_string(),
            input_properties: vec![],
            output_properties: vec![],
        });
    }
}
