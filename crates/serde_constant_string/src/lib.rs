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
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

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
}
