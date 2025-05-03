use crate::model::{
    ElementId, Function, GlobalType, GlobalTypeProperty, InputProperty, IntegerEnumElement,
    NumberEnumElement, OutputProperty, Package, Ref, Resource, StringEnumElement, Type,
};
use anyhow::{Context, Result};
use pulumi_gestalt_proto::pulumi_gestalt::pulumi_model as pulumi;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

/// Convert from our Rust Package to protobuf Package
pub fn package_to_proto(package: &Package) -> Result<pulumi::Package> {
    let mut proto_package = pulumi::Package {
        name: package.name.clone(),
        display_name: package.display_name.clone(),
        plugin_download_url: package.plugin_download_url.clone(),
        version: package.version.clone(),
        resources: HashMap::new(),
        functions: HashMap::new(),
        types: HashMap::new(),
    };

    // Convert resources
    for (id, resource) in &package.resources {
        proto_package.resources.insert(
            id.raw.clone(),
            resource_to_proto(resource).context("Failed to convert resource to proto")?,
        );
    }

    // Convert functions
    for (id, function) in &package.functions {
        proto_package.functions.insert(
            id.raw.clone(),
            function_to_proto(function).context("Failed to convert function to proto")?,
        );
    }

    // Convert types
    for (id, global_type) in &package.types {
        let global_type_value =
            global_type_to_proto(global_type).context("Failed to convert global type to proto")?;
        proto_package.types.insert(
            id.raw.clone(),
            pulumi::GlobalType {
                element_id: Some(element_id_to_proto(id)),
                global_type_value: Some(global_type_value),
            },
        );
    }

    Ok(proto_package)
}

/// Convert from protobuf Package to our Rust Package
pub fn proto_to_package(proto: &pulumi::Package) -> Result<Package> {
    // Convert resources
    let mut resources = BTreeMap::new();
    for proto_resource in proto.resources.values() {
        let resource =
            proto_to_resource(proto_resource).context("Failed to convert proto to resource")?;
        resources.insert(resource.element_id.clone(), resource);
    }

    // Convert functions
    let mut functions = BTreeMap::new();
    for proto_function in proto.functions.values() {
        let function =
            proto_to_function(proto_function).context("Failed to convert proto to function")?;
        functions.insert(function.element_id.clone(), function);
    }

    // Convert types
    let mut types = BTreeMap::new();
    for proto_type in proto.types.values() {
        let (id, global_type) =
            proto_to_global_type(proto_type).context("Failed to convert proto to global type")?;
        types.insert(id, global_type);
    }

    // Create the package
    Ok(Package::new(
        proto.name.clone(),
        proto.display_name.clone(),
        proto.plugin_download_url.clone(),
        proto.version.clone(),
        resources,
        functions,
        types,
    ))
}

// Helper functions for converting between Rust and protobuf types

fn resource_to_proto(resource: &Rc<Resource>) -> Result<pulumi::Resource> {
    let mut input_properties = Vec::new();
    for prop in &resource.input_properties {
        input_properties.push(input_property_to_proto(prop)?);
    }

    let mut output_properties = Vec::new();
    for prop in &resource.output_properties {
        output_properties.push(output_property_to_proto(prop)?);
    }

    Ok(pulumi::Resource {
        element_id: Some(element_id_to_proto(&resource.element_id)),
        description: resource.description.clone(),
        input_properties,
        output_properties,
    })
}

fn proto_to_resource(proto: &pulumi::Resource) -> Result<Resource> {
    let element_id = proto_to_element_id(
        proto
            .element_id
            .as_ref()
            .context("Resource proto missing element_id")?,
    );

    let mut input_properties = Vec::new();
    for prop in &proto.input_properties {
        input_properties.push(proto_to_input_property(prop)?);
    }

    let mut output_properties = Vec::new();
    for prop in &proto.output_properties {
        output_properties.push(proto_to_output_property(prop)?);
    }

    Ok(Resource {
        element_id,
        description: proto.description.clone(),
        input_properties,
        output_properties,
    })
}

fn function_to_proto(function: &Rc<Function>) -> Result<pulumi::Function> {
    let mut input_properties = Vec::new();
    for prop in &function.input_properties {
        input_properties.push(input_property_to_proto(prop)?);
    }

    let mut output_properties = Vec::new();
    for prop in &function.output_properties {
        output_properties.push(output_property_to_proto(prop)?);
    }

    Ok(pulumi::Function {
        element_id: Some(element_id_to_proto(&function.element_id)),
        description: function.description.clone(),
        input_properties,
        output_properties,
    })
}

fn proto_to_function(proto: &pulumi::Function) -> Result<Function> {
    let element_id = proto_to_element_id(
        proto
            .element_id
            .as_ref()
            .context("Function proto missing element_id")?,
    );

    let mut input_properties = Vec::new();
    for prop in &proto.input_properties {
        input_properties.push(proto_to_input_property(prop)?);
    }

    let mut output_properties = Vec::new();
    for prop in &proto.output_properties {
        output_properties.push(proto_to_output_property(prop)?);
    }

    Ok(Function {
        element_id,
        description: proto.description.clone(),
        input_properties,
        output_properties,
    })
}

fn element_id_to_proto(id: &ElementId) -> pulumi::ElementId {
    pulumi::ElementId {
        namespace: id.namespace.clone(),
        name: id.name.clone(),
        raw: id.raw.clone(),
    }
}

fn proto_to_element_id(proto: &pulumi::ElementId) -> ElementId {
    ElementId {
        namespace: proto.namespace.clone(),
        name: proto.name.clone(),
        raw: proto.raw.clone(),
    }
}

fn input_property_to_proto(prop: &InputProperty) -> Result<pulumi::InputProperty> {
    Ok(pulumi::InputProperty {
        name: prop.name.clone(),
        r#type: Some(type_to_proto(&prop.r#type)?),
        description: prop.description.clone(),
    })
}

fn proto_to_input_property(proto: &pulumi::InputProperty) -> Result<InputProperty> {
    Ok(InputProperty {
        name: proto.name.clone(),
        r#type: proto_to_type(
            proto
                .r#type
                .as_ref()
                .context("InputProperty proto missing type")?,
        )?,
        description: proto.description.clone(),
    })
}

fn output_property_to_proto(prop: &OutputProperty) -> Result<pulumi::OutputProperty> {
    Ok(pulumi::OutputProperty {
        name: prop.name.clone(),
        r#type: Some(type_to_proto(&prop.r#type)?),
        description: prop.description.clone(),
    })
}

fn proto_to_output_property(proto: &pulumi::OutputProperty) -> Result<OutputProperty> {
    Ok(OutputProperty {
        name: proto.name.clone(),
        r#type: proto_to_type(
            proto
                .r#type
                .as_ref()
                .context("OutputProperty proto missing type")?,
        )?,
        description: proto.description.clone(),
    })
}

fn type_to_proto(typ: &Type) -> Result<pulumi::Type> {
    let type_value = match typ {
        Type::Boolean => pulumi::r#type::TypeValue::Boolean(pulumi::Empty {}),
        Type::Integer => pulumi::r#type::TypeValue::Integer(pulumi::Empty {}),
        Type::Number => pulumi::r#type::TypeValue::Number(pulumi::Empty {}),
        Type::String => pulumi::r#type::TypeValue::String(pulumi::Empty {}),
        Type::Array(t) => pulumi::r#type::TypeValue::ArrayType(Box::new(type_to_proto(t)?)),
        Type::Object(t) => pulumi::r#type::TypeValue::ObjectType(Box::new(type_to_proto(t)?)),
        Type::Ref(r) => pulumi::r#type::TypeValue::RefType(ref_to_proto(r)?),
        Type::Option(t) => pulumi::r#type::TypeValue::OptionType(Box::new(type_to_proto(t)?)),
        Type::DiscriminatedUnion(types) => {
            let mut proto_types = Vec::new();
            for t in types {
                proto_types.push(type_to_proto(t)?);
            }
            pulumi::r#type::TypeValue::DiscriminatedUnion(pulumi::DiscriminatedUnion {
                types: proto_types,
            })
        }
        Type::ConstString(s) => pulumi::r#type::TypeValue::ConstString(s.clone()),
    };

    Ok(pulumi::Type {
        type_value: Some(type_value),
    })
}

fn proto_to_type(proto: &pulumi::Type) -> Result<Type> {
    let type_value = proto
        .type_value
        .as_ref()
        .context("Type proto missing type_value")?;

    match type_value {
        pulumi::r#type::TypeValue::Boolean(_) => Ok(Type::Boolean),
        pulumi::r#type::TypeValue::Integer(_) => Ok(Type::Integer),
        pulumi::r#type::TypeValue::Number(_) => Ok(Type::Number),
        pulumi::r#type::TypeValue::String(_) => Ok(Type::String),
        pulumi::r#type::TypeValue::ArrayType(t) => Ok(Type::Array(Box::new(proto_to_type(t)?))),
        pulumi::r#type::TypeValue::ObjectType(t) => Ok(Type::Object(Box::new(proto_to_type(t)?))),
        pulumi::r#type::TypeValue::RefType(r) => Ok(Type::Ref(proto_to_ref(r)?)),
        pulumi::r#type::TypeValue::OptionType(t) => Ok(Type::Option(Box::new(proto_to_type(t)?))),
        pulumi::r#type::TypeValue::DiscriminatedUnion(u) => {
            let mut types = Vec::new();
            for t in &u.types {
                types.push(proto_to_type(t)?);
            }
            Ok(Type::DiscriminatedUnion(types))
        }
        pulumi::r#type::TypeValue::ConstString(s) => Ok(Type::ConstString(s.clone())),
    }
}

fn ref_to_proto(r: &Ref) -> Result<pulumi::RefType> {
    let ref_value = match r {
        Ref::Type(id) => pulumi::ref_type::RefValue::TypeRef(element_id_to_proto(id)),
        Ref::Archive => pulumi::ref_type::RefValue::Archive(pulumi::Empty {}),
        Ref::Asset => pulumi::ref_type::RefValue::Asset(pulumi::Empty {}),
        Ref::Any => pulumi::ref_type::RefValue::Any(pulumi::Empty {}),
    };

    Ok(pulumi::RefType {
        ref_value: Some(ref_value),
    })
}

fn proto_to_ref(proto: &pulumi::RefType) -> Result<Ref> {
    let ref_value = proto
        .ref_value
        .as_ref()
        .context("RefType proto missing ref_value")?;

    match ref_value {
        pulumi::ref_type::RefValue::TypeRef(id) => Ok(Ref::Type(proto_to_element_id(id))),
        pulumi::ref_type::RefValue::Archive(_) => Ok(Ref::Archive),
        pulumi::ref_type::RefValue::Asset(_) => Ok(Ref::Asset),
        pulumi::ref_type::RefValue::Any(_) => Ok(Ref::Any),
    }
}

fn global_type_to_proto(global_type: &Rc<GlobalType>) -> Result<pulumi::GlobalTypeValue> {
    let global_type_value = match global_type.as_ref() {
        GlobalType::Object(desc, props) => {
            let mut proto_props = Vec::new();
            for prop in props {
                proto_props.push(global_type_property_to_proto(prop)?);
            }
            pulumi::global_type_value::Value::Object(pulumi::ObjectType {
                description: desc.clone(),
                properties: proto_props,
            })
        }
        GlobalType::StringEnum(desc, elements) => {
            let mut proto_elements = Vec::new();
            for elem in elements {
                proto_elements.push(string_enum_element_to_proto(elem));
            }
            pulumi::global_type_value::Value::StringEnum(pulumi::StringEnum {
                description: desc.clone(),
                elements: proto_elements,
            })
        }
        GlobalType::NumberEnum(desc, elements) => {
            let mut proto_elements = Vec::new();
            for elem in elements {
                proto_elements.push(number_enum_element_to_proto(elem));
            }
            pulumi::global_type_value::Value::NumberEnum(pulumi::NumberEnum {
                description: desc.clone(),
                elements: proto_elements,
            })
        }
        GlobalType::IntegerEnum(desc, elements) => {
            let mut proto_elements = Vec::new();
            for elem in elements {
                proto_elements.push(integer_enum_element_to_proto(elem));
            }
            pulumi::global_type_value::Value::IntegerEnum(pulumi::IntegerEnum {
                description: desc.clone(),
                elements: proto_elements,
            })
        }
    };

    Ok(pulumi::GlobalTypeValue {
        value: Some(global_type_value),
    })
}

fn proto_to_global_type(proto: &pulumi::GlobalType) -> Result<(ElementId, GlobalType)> {
    // Note: For simplicity, we're assuming we'll extract the element_id from elsewhere
    // This is a placeholder implementation
    // let element_id = ElementId::new("placeholder:id:Name")?;
    let element_id = proto.element_id.clone().unwrap();

    let global_type_value = proto
        .global_type_value
        .as_ref()
        .context("GlobalType proto missing global_type_value")?;

    let global_type = match &global_type_value.value {
        None => panic!("GlobalType value missing global_type_value"),
        Some(pulumi::global_type_value::Value::Object(obj)) => {
            let mut props = Vec::new();
            for prop in &obj.properties {
                props.push(proto_to_global_type_property(prop)?);
            }
            GlobalType::Object(obj.description.clone(), props)
        }
        Some(pulumi::global_type_value::Value::StringEnum(enum_type)) => {
            let mut elements = Vec::new();
            for elem in &enum_type.elements {
                elements.push(proto_to_string_enum_element(elem));
            }
            GlobalType::StringEnum(enum_type.description.clone(), elements)
        }
        Some(pulumi::global_type_value::Value::NumberEnum(enum_type)) => {
            let mut elements = Vec::new();
            for elem in &enum_type.elements {
                elements.push(proto_to_number_enum_element(elem));
            }
            GlobalType::NumberEnum(enum_type.description.clone(), elements)
        }
        Some(pulumi::global_type_value::Value::IntegerEnum(enum_type)) => {
            let mut elements = Vec::new();
            for elem in &enum_type.elements {
                elements.push(proto_to_integer_enum_element(elem));
            }
            GlobalType::IntegerEnum(enum_type.description.clone(), elements)
        }
    };

    Ok((proto_to_element_id(&element_id), global_type))
}

fn global_type_property_to_proto(prop: &GlobalTypeProperty) -> Result<pulumi::GlobalTypeProperty> {
    Ok(pulumi::GlobalTypeProperty {
        name: prop.name.clone(),
        r#type: Some(type_to_proto(&prop.r#type)?),
        description: prop.description.clone(),
    })
}

fn proto_to_global_type_property(proto: &pulumi::GlobalTypeProperty) -> Result<GlobalTypeProperty> {
    Ok(GlobalTypeProperty {
        name: proto.name.clone(),
        r#type: proto_to_type(
            proto
                .r#type
                .as_ref()
                .context("GlobalTypeProperty proto missing type")?,
        )?,
        description: proto.description.clone(),
    })
}

fn string_enum_element_to_proto(elem: &StringEnumElement) -> pulumi::StringEnumElement {
    pulumi::StringEnumElement {
        name: elem.name.clone(),
        value: elem.value.clone(),
        description: elem.description.clone(),
    }
}

fn proto_to_string_enum_element(proto: &pulumi::StringEnumElement) -> StringEnumElement {
    StringEnumElement {
        name: proto.name.clone(),
        value: proto.value.clone(),
        description: proto.description.clone(),
    }
}

fn number_enum_element_to_proto(elem: &NumberEnumElement) -> pulumi::NumberEnumElement {
    pulumi::NumberEnumElement {
        name: elem.name.clone(),
        value: elem.value,
        description: elem.description.clone(),
    }
}

fn proto_to_number_enum_element(proto: &pulumi::NumberEnumElement) -> NumberEnumElement {
    NumberEnumElement {
        name: proto.name.clone(),
        value: proto.value,
        description: proto.description.clone(),
    }
}

fn integer_enum_element_to_proto(elem: &IntegerEnumElement) -> pulumi::IntegerEnumElement {
    pulumi::IntegerEnumElement {
        name: elem.name.clone(),
        value: elem.value,
        description: elem.description.clone(),
    }
}

fn proto_to_integer_enum_element(proto: &pulumi::IntegerEnumElement) -> IntegerEnumElement {
    IntegerEnumElement {
        name: proto.name.clone(),
        value: proto.value,
        description: proto.description.clone(),
    }
}

#[cfg(test)]
mod tests {

    use crate::converter::package_to_proto;
    use crate::converter::proto_to_package;
    use crate::converter::tests::prop::arbitrary::any;
    use crate::model;
    use crate::model::Function;
    use crate::model::GlobalType;
    use crate::model::GlobalTypeProperty;
    use crate::model::InputProperty;
    use crate::model::OutputProperty;
    use crate::model::Ref;
    use crate::model::Type;
    use crate::model::{ElementId, Resource};
    use itertools::Itertools;
    use proptest::prelude::Just;
    use proptest::prelude::{Arbitrary, prop};
    use proptest::prelude::{BoxedStrategy, Strategy};
    use proptest::proptest;
    use proptest::{prop_assert_eq, prop_oneof};
    use std::collections::BTreeMap;

    proptest! {
        #[test]
        fn test_model_to_proto_to_model(
            original_package: model::Package,
        ) {
            let proto_package = package_to_proto(&original_package)
                .expect("Failed to convert model to protobuf");

            let converted_package = proto_to_package(&proto_package)
                .expect("Failed to convert protobuf to model");

            prop_assert_eq!(original_package, converted_package);
        }
    }

    pub fn string_strategy() -> impl Strategy<Value = String> {
        prop::collection::vec(
            prop::sample::select(
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
                    .chars()
                    .collect_vec(),
            ),
            1..=5,
        )
        .prop_map(|chars| chars.into_iter().collect::<String>())
    }

    pub fn map_strategy<K: Arbitrary + Ord, V: Arbitrary>() -> impl Strategy<Value = BTreeMap<K, V>>
    {
        prop::collection::btree_map(any::<K>(), any::<V>(), 0..=5)
    }

    pub fn vec_strategy<T: Arbitrary>() -> impl Strategy<Value = Vec<T>> {
        prop::collection::vec(any::<T>(), 0..=5)
    }

    impl Arbitrary for model::Package {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                string_strategy(),
                prop::option::of(string_strategy()),
                prop::option::of(string_strategy()),
                string_strategy(),
                vec_strategy::<Resource>(),
                vec_strategy::<Function>(),
                map_strategy::<ElementId, GlobalType>()
                    .prop_map(|m| m.into_iter().collect::<BTreeMap<_, _>>()),
            )
                .prop_map(
                    |(
                        name,
                        display_name,
                        plugin_download_url,
                        version,
                        resources,
                        functions,
                        types,
                    )| {
                        let mut resource_name_map = BTreeMap::new();
                        for resource in resources {
                            resource_name_map.insert(resource.element_id.clone(), resource);
                        }

                        let mut function_name_map = BTreeMap::new();
                        for function in functions {
                            function_name_map.insert(function.element_id.clone(), function);
                        }

                        model::Package::new(
                            name,
                            display_name,
                            plugin_download_url,
                            version,
                            resource_name_map,
                            function_name_map,
                            types,
                        )
                    },
                )
                .boxed()
        }
    }

    impl Arbitrary for Type {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            let leaf = prop_oneof![
                Just(Type::Boolean),
                Just(Type::Integer),
                Just(Type::Number),
                Just(Type::String),
                string_strategy().prop_map(Type::ConstString),
                any::<Ref>().prop_map(Type::Ref)
            ];

            leaf.prop_recursive(4, 20, 10, |inner| {
                prop_oneof![
                    inner.clone().prop_map(|t| Type::Array(Box::new(t))),
                    inner.clone().prop_map(|t| Type::Object(Box::new(t))),
                    inner.clone().prop_map(|t| Type::Option(Box::new(t))),
                    prop::collection::vec(inner, 1..5).prop_map(Type::DiscriminatedUnion)
                ]
            })
            .boxed()
        }
    }

    impl Arbitrary for InputProperty {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                string_strategy(),
                any::<Type>(),
                prop::option::of(string_strategy()),
            )
                .prop_map(|(name, r#type, description)| InputProperty {
                    name,
                    r#type,
                    description,
                })
                .boxed()
        }
    }

    impl Arbitrary for OutputProperty {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                string_strategy(),
                any::<Type>(),
                prop::option::of(string_strategy()),
            )
                .prop_map(|(name, r#type, description)| OutputProperty {
                    name,
                    r#type,
                    description,
                })
                .boxed()
        }
    }

    impl Arbitrary for GlobalTypeProperty {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                string_strategy(),
                any::<Type>(),
                prop::option::of(string_strategy()),
            )
                .prop_map(|(name, r#type, description)| GlobalTypeProperty {
                    name,
                    r#type,
                    description,
                })
                .boxed()
        }
    }

    impl Arbitrary for ElementId {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                string_strategy(),
                prop::collection::vec(string_strategy(), 0..=3),
                string_strategy(),
            )
                .prop_map(|(raw, namespace, name)| ElementId {
                    raw,
                    namespace,
                    name,
                })
                .boxed()
        }
    }

    impl Arbitrary for Ref {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(Ref::Archive),
                Just(Ref::Asset),
                Just(Ref::Any),
                any::<ElementId>().prop_map(Ref::Type)
            ]
            .boxed()
        }
    }
    
}
