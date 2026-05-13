//! This crate provides a macro to generate a constant string type that can be used with Serde for serialization and deserialization.
//! The generated type will serialize to a specific string value and will only deserialize from that exact string value.
//!
//! # Examples
//!
//! ```
//! # use serde::{Deserialize, Serialize};
//! # use serde_json::json;
//! use pulumi_gestalt_serde_constant_string::generate_string_const;
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!    tpe: IntegerString,
//!    value: i32,
//! }
//! generate_string_const!(IntegerString, "Integer");
//!
//! assert!(serde_json::from_value::<MyStruct>(json!({ "tpe": "Integer", "value": 1 })).is_ok());
//! assert!(serde_json::from_value::<MyStruct>(json!({ "tpe": "Double", "value": 2 })).is_err());
//! ```

#[doc(hidden)]
pub mod __private {
    pub use pulumi_gestalt_model;
    pub use rootcause;
}

#[doc(hidden)]
#[macro_export]
macro_rules! generate_string_const {
    ($struct_name:ident, $constant:tt) => {
        #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
        pub(crate) struct $struct_name;
        impl Default for $struct_name {
            fn default() -> Self {
                Self {}
            }
        }

        impl serde::Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str($constant)
            }
        }

        impl<'de> serde::Deserialize<'de> for $struct_name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct ConstantVisitor;

                impl<'de> serde::de::Visitor<'de> for ConstantVisitor {
                    type Value = $struct_name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(formatter, "the string '{}'", $constant)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        if value == $constant {
                            Ok($struct_name {})
                        } else {
                            Err(serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(value),
                                &self,
                            ))
                        }
                    }
                }

                deserializer.deserialize_str(ConstantVisitor {})
            }
        }

        impl $crate::__private::pulumi_gestalt_model::FromPulumiValue for $struct_name {
            fn from_pulumi_value(
                value: &$crate::__private::pulumi_gestalt_model::PulumiValue,
            ) -> $crate::__private::rootcause::Result<Self> {
                use $crate::__private::pulumi_gestalt_model::PulumiValueContent;
                use $crate::__private::rootcause::bail;

                match &value.content {
                    PulumiValueContent::String(s) if s == $constant => Ok($struct_name {}),
                    PulumiValueContent::String(s) => bail!(
                        "Expected string '{}', got '{}'",
                        $constant,
                        s
                    ),
                    _ => bail!("Expected String, got {:?}", value.content),
                }
            }
        }

        impl $crate::__private::pulumi_gestalt_model::ToPulumiValue for $struct_name {
            fn to_pulumi_value(
                &self,
            ) -> impl std::future::Future<
                Output = $crate::__private::pulumi_gestalt_model::PulumiValue,
            > {
                async move {
                    $crate::__private::pulumi_gestalt_model::PulumiValue {
                        content: $crate::__private::pulumi_gestalt_model::PulumiValueContent::String(
                            $constant.to_string(),
                        ),
                        secret: false,
                        dependencies: std::collections::HashSet::new(),
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use pulumi_gestalt_model::{
        FromPulumiValue as FromPulumiValueTrait, PulumiValue, PulumiValueContent,
        ToPulumiValue as ToPulumiValueTrait,
    };
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;

    #[derive(Serialize, Deserialize)]
    struct MyStruct {
        tpe: StringConstants,
        age: i32,
    }
    generate_string_const!(StringConstants, "HELLO WORLD");

    #[test]
    fn string_const_should_serialize() {
        let my_struct = MyStruct {
            tpe: StringConstants,
            age: 0,
        };
        assert_eq!(
            serde_json::to_string(&my_struct).unwrap(),
            r#"{"tpe":"HELLO WORLD","age":0}"#
        );
    }

    #[test]
    fn string_const_should_deserialize() {
        let my_struct: MyStruct = serde_json::from_str(r#"{"tpe":"HELLO WORLD","age":0}"#).unwrap();
        assert_eq!(my_struct.tpe, StringConstants);
        assert_eq!(my_struct.age, 0);
    }

    #[test]
    fn string_const_should_fail_to_deserialize_invalid_value() {
        let result: Result<MyStruct, _> = serde_json::from_str(r#"{"tpe":"INVALID","age":0}"#);
        assert!(result.is_err());
    }

    #[test]
    fn string_const_should_to_pulumi_value() {
        let value = block_on(StringConstants.to_pulumi_value());
        assert_eq!(
            value.content,
            PulumiValueContent::String("HELLO WORLD".to_string())
        );
        assert!(!value.secret);
        assert!(value.dependencies.is_empty());
    }

    #[test]
    fn string_const_should_from_pulumi_value() {
        let value = PulumiValue {
            content: PulumiValueContent::String("HELLO WORLD".to_string()),
            secret: false,
            dependencies: HashSet::new(),
        };

        let result = StringConstants::from_pulumi_value(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StringConstants);
    }

    #[test]
    fn string_const_should_fail_from_pulumi_value_on_invalid_value() {
        let value = PulumiValue {
            content: PulumiValueContent::String("INVALID".to_string()),
            secret: false,
            dependencies: HashSet::new(),
        };

        let result = StringConstants::from_pulumi_value(&value);
        assert!(result.is_err());
    }
}
