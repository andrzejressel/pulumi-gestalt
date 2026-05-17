use crate::Output;
use anyhow::{Context as AnyhowContext, Result};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

/// Represents a dynamically typed Pulumi value.
///
/// Serialization and deserialization are fully proxied to the wrapped value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PulumiAny(pub(crate) Value);

impl PulumiAny {
    /// Build [`PulumiAny`] from any serializable value.
    pub fn from_serializable<T>(value: T) -> Result<Self>
    where
        T: Serialize,
    {
        let value =
            serde_json::to_value(value).context("Failed to serialize value into `PulumiAny`")?;
        Ok(Self(value))
    }

    /// Deserialize the wrapped value into a concrete type.
    pub fn deserialize_as<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        serde_json::from_value(self.0.clone())
            .context("Failed to deserialize `PulumiAny` into requested type")
    }
}

impl From<Value> for PulumiAny {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl From<PulumiAny> for Value {
    fn from(value: PulumiAny) -> Self {
        value.0
    }
}

/// Construct a [`PulumiAny`] value from a JSON literal.
///
/// The syntax is identical to [`serde_json::json!`] — any valid JSON literal is accepted,
/// including objects, arrays, strings, numbers, booleans, and `null`.
///
/// # Examples
///
/// ```
/// use pulumi_gestalt_rust::{PulumiAny, pulumi_any};
///
/// let v: PulumiAny = pulumi_any!({"name": "Alice", "count": 3});
/// ```
#[macro_export]
macro_rules! pulumi_any {
    ($($tt:tt)*) => {
        $crate::PulumiAny::from(::serde_json::json!($($tt)*))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! pulumi_any_v2_internal {
    (null) => {
        $crate::__private::pulumi_gestalt_model::__private::pulumi_value_output(
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
        $crate::__private::pulumi_gestalt_model::__private::to_pulumi_value_output($other)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! pulumi_any_v2_internal_array {
    ([$($elems:expr,)*]) => {
        $crate::__private::pulumi_gestalt_model::__private::pulumi_value_output_array(
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
        $crate::__private::pulumi_gestalt_model::__private::pulumi_value_output_object(
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

/// Construct an [`pulumi_gestalt_model::Output<pulumi_gestalt_model::PulumiValue>`] from
/// JSON-like literals.
///
/// The syntax is serde_json-style and supports nested arrays/objects, trailing commas,
/// and values that are already `pulumi_gestalt_model::Output<T>`.
#[macro_export]
macro_rules! pulumi_any_v2 {
    ($($tt:tt)+) => {
        $crate::pulumi_any_v2_internal!($($tt)+)
    };
}

/// Extension trait for converting values to [`PulumiAny`].
pub trait ToPulumiAny {
    /// The converted output type.
    type Converted;

    /// Convert value to [`PulumiAny`] or [`Output<PulumiAny>`].
    ///
    /// Panics when serialization fails.
    fn to_pulumi_any(self) -> Self::Converted;
}

impl<T> ToPulumiAny for T
where
    T: Serialize,
{
    type Converted = PulumiAny;

    fn to_pulumi_any(self) -> Self::Converted {
        PulumiAny::from_serializable(self)
            .expect("Failed to serialize value while converting to `PulumiAny`")
    }
}

impl<T> ToPulumiAny for Output<T>
where
    T: Serialize + DeserializeOwned,
{
    type Converted = Output<PulumiAny>;

    fn to_pulumi_any(self) -> Self::Converted {
        self.map(|value| value.to_pulumi_any())
    }
}

#[cfg(test)]
mod tests {
    use super::{PulumiAny, ToPulumiAny};
    use pulumi_gestalt_model::__private::futures::executor::block_on;
    use pulumi_gestalt_model::{Output as ModelOutput, PulumiValueContent, ToPulumiValue};
    use serde::Serialize;

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct ExampleInput {
        name: String,
        count: i32,
    }

    struct FailingSerialize;

    impl Serialize for FailingSerialize {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("serialization failed"))
        }
    }

    #[test]
    fn pulumi_any_roundtrip() {
        let value = PulumiAny::from_serializable(ExampleInput {
            name: "roundtrip".to_string(),
            count: 10,
        })
        .unwrap();

        let decoded: ExampleInput = value.deserialize_as().unwrap();
        assert_eq!(
            decoded,
            ExampleInput {
                name: "roundtrip".to_string(),
                count: 10,
            }
        );
    }

    #[test]
    fn to_pulumi_any_converts_plain_value() {
        let value = ExampleInput {
            name: "demo".to_string(),
            count: 7,
        };
        let pulumi_any = value.to_pulumi_any();
        let decoded: ExampleInput = pulumi_any.deserialize_as().unwrap();
        assert_eq!(
            decoded,
            ExampleInput {
                name: "demo".to_string(),
                count: 7,
            }
        );
    }

    #[test]
    #[should_panic(expected = "Failed to serialize value while converting to `PulumiAny`")]
    fn to_pulumi_any_panics_for_invalid_payload() {
        let _ = FailingSerialize.to_pulumi_any();
    }

    #[test]
    fn pulumi_any_macro_creates_value() {
        let v = pulumi_any!({"name": "macro_test", "count": 5});
        let decoded: ExampleInput = v.deserialize_as().unwrap();
        assert_eq!(
            decoded,
            ExampleInput {
                name: "macro_test".to_string(),
                count: 5,
            }
        );
    }

    #[test]
    fn pulumi_any_macro_array() {
        let v = pulumi_any!([1, 2, 3]);
        let decoded: Vec<i32> = v.deserialize_as().unwrap();
        assert_eq!(decoded, vec![1, 2, 3]);
    }

    #[test]
    fn pulumi_any_macro_scalar() {
        let v = pulumi_any!(42);
        let decoded: i32 = v.deserialize_as().unwrap();
        assert_eq!(decoded, 42);
    }

    #[test]
    fn pulumi_any_v2_macro_scalar() {
        let integer = block_on(pulumi_any_v2!(42).to_pulumi_value());
        assert_eq!(integer.content, PulumiValueContent::Integer(42));

        let float = block_on(pulumi_any_v2!(1.5f64).to_pulumi_value());
        assert_eq!(float.content, PulumiValueContent::Number(1.5));

        let boolean = block_on(pulumi_any_v2!(true).to_pulumi_value());
        assert_eq!(boolean.content, PulumiValueContent::Boolean(true));

        let string = block_on(pulumi_any_v2!("hello").to_pulumi_value());
        assert_eq!(
            string.content,
            PulumiValueContent::String("hello".to_string())
        );
    }

    #[test]
    fn pulumi_any_v2_macro_array_and_object() {
        let array = block_on(pulumi_any_v2!([1, 2, 3]).to_pulumi_value());
        match array.content {
            PulumiValueContent::Array(values) => assert_eq!(values.len(), 3),
            _ => panic!("Expected array content"),
        }

        let object = block_on(pulumi_any_v2!({"name": "macro_test", "count": 5}).to_pulumi_value());
        match object.content {
            PulumiValueContent::Object(values) => assert_eq!(values.len(), 2),
            _ => panic!("Expected object content"),
        }
    }

    #[test]
    fn pulumi_any_v2_macro_nested_and_trailing_commas() {
        let value = block_on(
            pulumi_any_v2!({
                "items": [
                    1,
                    {"name": "a",},
                ],
                "ok": true,
            })
            .to_pulumi_value(),
        );

        match value.content {
            PulumiValueContent::Object(values) => {
                assert_eq!(values.len(), 2);
            }
            _ => panic!("Expected object content"),
        }
    }

    #[test]
    fn pulumi_any_v2_macro_nested_model_outputs() {
        let value = block_on(
            pulumi_any_v2!({
                "known": ModelOutput::new(7i32),
                "secret": ModelOutput::new_secret("sensitive"),
                "unknown": ModelOutput::<i32>::new_nothing(),
                "nested": [ModelOutput::new(true)],
            })
            .to_pulumi_value(),
        );

        assert!(value.secret);
        assert!(value.dependencies.is_empty());

        match value.content {
            PulumiValueContent::Object(fields) => {
                let unknown = fields.iter().find(|(k, _)| k == "unknown").unwrap();
                assert_eq!(unknown.1.content, PulumiValueContent::Nothing);
            }
            _ => panic!("Expected object content"),
        }
    }
}
