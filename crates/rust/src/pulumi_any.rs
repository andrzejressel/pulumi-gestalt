use pulumi_gestalt_model::ToPulumiValue;

pub type PulumiAny = pulumi_gestalt_model::PulumiValueMiddleware;

/// Construct a [`PulumiAny`] value from JSON-like literals.
///
/// The syntax supports nested arrays/objects, trailing commas,
/// and values implementing [`pulumi_gestalt_model::ToPulumiValue`].
#[macro_export]
macro_rules! pulumi_any {
    ($($tt:tt)+) => {
        $crate::pulumi_any_v2_internal!($($tt)+)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! pulumi_any_v2_internal {
    (null) => {
        $crate::__private::pulumi_gestalt_model::__private::pulumi_value_middleware(
            $crate::__private::pulumi_gestalt_model::PulumiValueContent::None,
        )
    };
    ([$($tt:tt)*]) => {
        $crate::pulumi_any_v2_internal_array!([] $($tt)*)
    };
    ({$($tt:tt)*}) => {
        $crate::pulumi_any_v2_internal_object!([] $($tt)*)
    };
    ($other:expr) => {
        $crate::__private::pulumi_gestalt_model::__private::to_pulumi_value_middleware($other)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! pulumi_any_v2_internal_array {
    ([$($elems:expr,)*]) => {
        $crate::__private::pulumi_gestalt_model::__private::pulumi_value_middleware_array(
            vec![$($elems,)*]
        )
    };
    ([$($elems:expr,)*] , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_array!([$($elems,)*] $($rest)*)
    };
    ([$($elems:expr,)*] null , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!(null),
            ]
            $($rest)*
        )
    };
    ([$($elems:expr,)*] [$($inner:tt)*] , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!([$($inner)*]),
            ]
            $($rest)*
        )
    };
    ([$($elems:expr,)*] {$($inner:tt)*} , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!({$($inner)*}),
            ]
            $($rest)*
        )
    };
    ([$($elems:expr,)*] $next:expr , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!($next),
            ]
            $($rest)*
        )
    };
    ([$($elems:expr,)*] null) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!(null),
            ]
        )
    };
    ([$($elems:expr,)*] [$($inner:tt)*]) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!([$($inner)*]),
            ]
        )
    };
    ([$($elems:expr,)*] {$($inner:tt)*}) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!({$($inner)*}),
            ]
        )
    };
    ([$($elems:expr,)*] $next:expr) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!($next),
            ]
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! pulumi_any_v2_internal_object {
    ([$($fields:expr,)*]) => {
        $crate::__private::pulumi_gestalt_model::__private::pulumi_value_middleware_object(
            vec![$($fields,)*]
        )
    };
    ([$($fields:expr,)*] , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_object!([$($fields,)*] $($rest)*)
    };
    ([$($fields:expr,)*] $key:tt : null , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!(null),
                ),
            ]
            $($rest)*
        )
    };
    ([$($fields:expr,)*] $key:tt : [$($inner:tt)*] , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!([$($inner)*]),
                ),
            ]
            $($rest)*
        )
    };
    ([$($fields:expr,)*] $key:tt : {$($inner:tt)*} , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!({$($inner)*}),
                ),
            ]
            $($rest)*
        )
    };
    ([$($fields:expr,)*] $key:tt : $value:expr , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!($value),
                ),
            ]
            $($rest)*
        )
    };
    ([$($fields:expr,)*] $key:tt : null) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!(null),
                ),
            ]
        )
    };
    ([$($fields:expr,)*] $key:tt : [$($inner:tt)*]) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!([$($inner)*]),
                ),
            ]
        )
    };
    ([$($fields:expr,)*] $key:tt : {$($inner:tt)*}) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!({$($inner)*}),
                ),
            ]
        )
    };
    ([$($fields:expr,)*] $key:tt : $value:expr) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!($value),
                ),
            ]
        )
    };
}

pub trait ToPulumiAny {
    fn to_pulumi_any(self) -> PulumiAny;
}

impl<T> ToPulumiAny for T
where
    T: ToPulumiValue + Send + Sync + 'static,
{
    fn to_pulumi_any(self) -> PulumiAny {
        pulumi_gestalt_model::__private::to_pulumi_value_middleware(self)
    }
}

#[cfg(test)]
mod tests {
    use super::ToPulumiAny;
    use pulumi_gestalt_model::__private::futures::executor::block_on;
    use pulumi_gestalt_model::{
        Output as ModelOutput, PulumiValue, PulumiValueContent, ToPulumiValue,
    };
    use std::collections::HashSet;

    struct CustomToPulumiValue {
        id: i32,
    }

    impl ToPulumiValue for CustomToPulumiValue {
        fn to_pulumi_value(&self) -> impl std::future::Future<Output = PulumiValue> + Send {
            let id = self.id;
            std::future::ready(PulumiValue {
                content: PulumiValueContent::Object(vec![(
                    "custom-id".to_string(),
                    pv(PulumiValueContent::Integer(id + 1000)),
                )]),
                secret: false,
                dependencies: HashSet::new(),
            })
        }
    }

    fn pv(content: PulumiValueContent) -> PulumiValue {
        PulumiValue {
            content,
            secret: false,
            dependencies: HashSet::new(),
        }
    }

    fn pvs(content: PulumiValueContent) -> PulumiValue {
        PulumiValue {
            content,
            secret: true,
            dependencies: HashSet::new(),
        }
    }

    #[test]
    fn to_pulumi_any_converts_plain_value() {
        let pulumi_any = "demo".to_pulumi_any();
        let decoded = block_on(pulumi_any.to_pulumi_value());
        assert_eq!(decoded, pv(PulumiValueContent::String("demo".to_string())));
    }

    #[test]
    fn pulumi_any_macro_scalar() {
        let integer = block_on(pulumi_any!(42).to_pulumi_value());
        assert_eq!(integer, pv(PulumiValueContent::Integer(42)));

        let float = block_on(pulumi_any!(1.5f64).to_pulumi_value());
        assert_eq!(float, pv(PulumiValueContent::Number(1.5)));

        let boolean = block_on(pulumi_any!(true).to_pulumi_value());
        assert_eq!(boolean, pv(PulumiValueContent::Boolean(true)));

        let string = block_on(pulumi_any!("hello").to_pulumi_value());
        assert_eq!(string, pv(PulumiValueContent::String("hello".to_string())));
    }

    #[test]
    fn pulumi_any_macro_array_and_object() {
        let array = block_on(pulumi_any!([1, 2, 3]).to_pulumi_value());
        assert_eq!(
            array,
            pv(PulumiValueContent::Array(vec![
                pv(PulumiValueContent::Integer(1)),
                pv(PulumiValueContent::Integer(2)),
                pv(PulumiValueContent::Integer(3)),
            ]))
        );

        let object = block_on(pulumi_any!({"name": "macro_test", "count": 5}).to_pulumi_value());
        assert_eq!(
            object,
            pv(PulumiValueContent::Object(vec![
                ("count".to_string(), pv(PulumiValueContent::Integer(5))),
                (
                    "name".to_string(),
                    pv(PulumiValueContent::String("macro_test".to_string()))
                ),
            ]))
        );
    }

    #[test]
    fn pulumi_any_macro_nested_and_trailing_commas() {
        let value = block_on(
            pulumi_any!({
                "items": [
                    1,
                    {"name": "a",},
                ],
                "ok": true,
            })
            .to_pulumi_value(),
        );

        assert_eq!(
            value,
            pv(PulumiValueContent::Object(vec![
                (
                    "items".to_string(),
                    pv(PulumiValueContent::Array(vec![
                        pv(PulumiValueContent::Integer(1)),
                        pv(PulumiValueContent::Object(vec![(
                            "name".to_string(),
                            pv(PulumiValueContent::String("a".to_string())),
                        )])),
                    ])),
                ),
                ("ok".to_string(), pv(PulumiValueContent::Boolean(true))),
            ]))
        );
    }

    #[test]
    fn pulumi_any_macro_nested_model_outputs() {
        let value = block_on(
            pulumi_any!({
                "known": ModelOutput::new(7i32),
                "secret": ModelOutput::new_secret("sensitive"),
                "unknown": ModelOutput::<i32>::new_nothing(),
                "nested": [ModelOutput::new(true)],
            })
            .to_pulumi_value(),
        );

        assert_eq!(
            value,
            pvs(PulumiValueContent::Object(vec![
                ("known".to_string(), pv(PulumiValueContent::Integer(7))),
                (
                    "nested".to_string(),
                    pv(PulumiValueContent::Array(vec![pv(
                        PulumiValueContent::Boolean(true)
                    )])),
                ),
                (
                    "secret".to_string(),
                    pvs(PulumiValueContent::String("sensitive".to_string())),
                ),
                ("unknown".to_string(), pv(PulumiValueContent::Nothing)),
            ]))
        );
    }

    #[test]
    fn pulumi_any_macro_object_sorts_keys() {
        let value = block_on(
            pulumi_any!({
                "z": 1,
                "a": 2,
            })
            .to_pulumi_value(),
        );

        assert_eq!(
            value,
            pv(PulumiValueContent::Object(vec![
                ("a".to_string(), pv(PulumiValueContent::Integer(2))),
                ("z".to_string(), pv(PulumiValueContent::Integer(1))),
            ]))
        );
    }

    #[test]
    fn pulumi_any_macro_object_last_wins_for_duplicate_keys() {
        let value = block_on(
            pulumi_any!({
                "a": 1,
                "a": 2,
                "b": 3,
            })
            .to_pulumi_value(),
        );

        assert_eq!(
            value,
            pv(PulumiValueContent::Object(vec![
                ("a".to_string(), pv(PulumiValueContent::Integer(2))),
                ("b".to_string(), pv(PulumiValueContent::Integer(3))),
            ]))
        );
    }

    #[test]
    fn pulumi_any_macro_array_propagates_secret_from_nested_output() {
        let value = block_on(
            pulumi_any!([ModelOutput::new(1i32), ModelOutput::new_secret(2i32),]).to_pulumi_value(),
        );

        assert_eq!(
            value,
            pvs(PulumiValueContent::Array(vec![
                pv(PulumiValueContent::Integer(1)),
                pvs(PulumiValueContent::Integer(2)),
            ]))
        );
    }

    #[test]
    fn pulumi_any_macro_uses_custom_to_pulumi_value_impl() {
        let value = block_on(
            pulumi_any!({
                "custom": CustomToPulumiValue { id: 7 },
            })
            .to_pulumi_value(),
        );

        assert_eq!(
            value,
            pv(PulumiValueContent::Object(vec![(
                "custom".to_string(),
                pv(PulumiValueContent::Object(vec![(
                    "custom-id".to_string(),
                    pv(PulumiValueContent::Integer(1007)),
                )])),
            )]))
        );
    }
}
