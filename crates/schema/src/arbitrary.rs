use crate::model;
use crate::model::Function;
use crate::model::GlobalTypeProperty;
use crate::model::GlobalTypeValue;
use crate::model::InputProperty;
use crate::model::NumberEnumElement;
use crate::model::OutputProperty;
use crate::model::Ref;
use crate::model::StringEnumElement;
use crate::model::*;
use crate::model::{ElementId, Resource};
use itertools::Itertools;
use proptest::arbitrary::any;
use proptest::prelude::Just;
use proptest::prelude::{Arbitrary, prop};
use proptest::prelude::{BoxedStrategy, Strategy};
use proptest::prop_oneof;
use std::collections::BTreeMap;

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
            vec_strategy::<GlobalType>(),
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

                    let mut types_map = BTreeMap::new();
                    for global_type in types {
                        types_map.insert(global_type.element_id.clone(), global_type);
                    }

                    model::Package::new(
                        name,
                        display_name,
                        plugin_download_url,
                        version,
                        resource_name_map,
                        function_name_map,
                        types_map,
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

impl Arbitrary for GlobalTypeValue {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            (
                prop::option::of(string_strategy()),
                vec_strategy::<GlobalTypeProperty>()
            )
                .prop_map(|(desc, props)| GlobalTypeValue::Object(desc, props)),
            (
                prop::option::of(string_strategy()),
                vec_strategy::<StringEnumElement>()
            )
                .prop_map(|(desc, elements)| GlobalTypeValue::StringEnum(desc, elements)),
            (
                prop::option::of(string_strategy()),
                vec_strategy::<NumberEnumElement>()
            )
                .prop_map(|(desc, elements)| GlobalTypeValue::NumberEnum(desc, elements)),
            (
                prop::option::of(string_strategy()),
                vec_strategy::<IntegerEnumElement>()
            )
                .prop_map(|(desc, elements)| GlobalTypeValue::IntegerEnum(desc, elements))
        ]
        .boxed()
    }
}

impl Arbitrary for GlobalType {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (any::<ElementId>(), any::<GlobalTypeValue>())
            .prop_map(|(element_id, value)| GlobalType { element_id, value })
            .boxed()
    }
}

impl Arbitrary for StringEnumElement {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (
            string_strategy(),
            string_strategy(),
            prop::option::of(string_strategy()),
        )
            .prop_map(|(name, value, description)| StringEnumElement {
                name,
                value,
                description,
            })
            .boxed()
    }
}

impl Arbitrary for NumberEnumElement {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (
            string_strategy(),
            prop::num::f64::NORMAL,
            prop::option::of(string_strategy()),
        )
            .prop_map(|(name, value, description)| NumberEnumElement {
                name,
                value,
                description,
            })
            .boxed()
    }
}

impl Arbitrary for IntegerEnumElement {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (
            string_strategy(),
            prop::num::i64::ANY,
            prop::option::of(string_strategy()),
        )
            .prop_map(|(name, value, description)| IntegerEnumElement {
                name,
                value,
                description,
            })
            .boxed()
    }
}

impl Arbitrary for Resource {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (
            any::<ElementId>(),
            prop::option::of(string_strategy()),
            vec_strategy::<InputProperty>(),
            vec_strategy::<OutputProperty>(),
        )
            .prop_map(
                |(element_id, description, input_properties, output_properties)| Resource {
                    element_id,
                    description,
                    input_properties,
                    output_properties,
                },
            )
            .boxed()
    }
}

impl Arbitrary for Function {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (
            any::<ElementId>(),
            prop::option::of(string_strategy()),
            vec_strategy::<InputProperty>(),
            vec_strategy::<OutputProperty>(),
        )
            .prop_map(
                |(element_id, description, input_properties, output_properties)| Function {
                    element_id,
                    description,
                    input_properties,
                    output_properties,
                },
            )
            .boxed()
    }
}
