//! This crate provides a macro to generate a constant string type that can be used with Serde
//! and Pulumi value conversion traits.

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

        impl $crate::__private::pulumi_gestalt_model::FromPulumiValue for $struct_name {
            fn from_pulumi_value(
                value: &$crate::__private::pulumi_gestalt_model::PulumiValue,
            ) -> $crate::__private::rootcause::Result<Self> {
                use $crate::__private::pulumi_gestalt_model::PulumiValueContent;
                use $crate::__private::rootcause::bail;

                match &value.content {
                    PulumiValueContent::String(s) if s == $constant => Ok($struct_name {}),
                    PulumiValueContent::String(s) => {
                        bail!("Expected string '{}', got '{}'", $constant, s)
                    }
                    _ => bail!("Expected String, got {:?}", value.content),
                }
            }
        }

        impl $crate::__private::pulumi_gestalt_model::ToPulumiValue for $struct_name {
            fn to_pulumi_value(
                &self,
            ) -> impl std::future::Future<
                Output = $crate::__private::pulumi_gestalt_model::PulumiValue,
            > + Send {
                async move {
                    $crate::__private::pulumi_gestalt_model::PulumiValue {
                        content:
                            $crate::__private::pulumi_gestalt_model::PulumiValueContent::String(
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
    use std::collections::HashSet;

    generate_string_const!(StringConstants, "HELLO WORLD");

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
        assert_eq!(result.expect("conversion should work"), StringConstants);
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
