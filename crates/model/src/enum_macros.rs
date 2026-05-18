#[macro_export]
#[doc(hidden)]
macro_rules! pulumi_value_string_enum {
    (
        enum $enum_name:ident {
            $($variant:ident => $value:literal),* $(,)?
        }
    ) => {
        impl $crate::ToPulumiValue for $enum_name {
            fn to_pulumi_value(
                &self,
            ) -> impl std::future::Future<Output = $crate::PulumiValue> + Send {
                let value = match self {
                    $($enum_name::$variant => $value.to_string()),*
                };
                std::future::ready($crate::PulumiValue {
                    content: $crate::PulumiValueContent::String(value),
                    secret: false,
                    dependencies: std::collections::HashSet::new(),
                })
            }
        }

        impl $crate::FromPulumiValue for $enum_name {
            fn from_pulumi_value(value: &$crate::PulumiValue) -> $crate::__private::rootcause::Result<Self> {
                use $crate::__private::rootcause::bail;
                use $crate::PulumiValueContent;
                match &value.content {
                    PulumiValueContent::String(s) => {
                        match s.as_str() {
                            $($value => Ok($enum_name::$variant)),*,
                            _ => bail!("Invalid string enum value: {}", s),
                        }
                    }
                    _ => bail!("Expected String, got {:?}", value.content),
                }
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! pulumi_value_integer_enum {
    (
        enum $enum_name:ident {
            $($variant:ident => $value:literal),* $(,)?
        }
    ) => {
        impl $crate::ToPulumiValue for $enum_name {
            fn to_pulumi_value(
                &self,
            ) -> impl std::future::Future<Output = $crate::PulumiValue> + Send {
                let value: i32 = match self {
                    $($enum_name::$variant => $value),*
                };
                std::future::ready($crate::PulumiValue {
                    content: $crate::PulumiValueContent::Integer(value),
                    secret: false,
                    dependencies: std::collections::HashSet::new(),
                })
            }
        }

        impl $crate::FromPulumiValue for $enum_name {
            fn from_pulumi_value(value: &$crate::PulumiValue) -> $crate::__private::rootcause::Result<Self> {
                use $crate::__private::rootcause::bail;
                use $crate::PulumiValueContent;
                match &value.content {
                    PulumiValueContent::Integer(i) => {
                        match i {
                            $($value => Ok($enum_name::$variant)),*,
                            _ => bail!("Invalid integer enum value: {}", i),
                        }
                    }
                    _ => bail!("Expected Integer, got {:?}", value.content),
                }
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! pulumi_value_number_enum {
    (
        enum $enum_name:ident {
            $($variant:ident => $value:literal),* $(,)?
        }
    ) => {
        impl $crate::ToPulumiValue for $enum_name {
            fn to_pulumi_value(
                &self,
            ) -> impl std::future::Future<Output = $crate::PulumiValue> + Send {
                let value: f64 = match self {
                    $($enum_name::$variant => $value),*
                };
                std::future::ready($crate::PulumiValue {
                    content: $crate::PulumiValueContent::Number(value),
                    secret: false,
                    dependencies: std::collections::HashSet::new(),
                })
            }
        }

        impl $crate::FromPulumiValue for $enum_name {
            fn from_pulumi_value(value: &$crate::PulumiValue) -> $crate::__private::rootcause::Result<Self> {
                use $crate::__private::rootcause::bail;
                use $crate::PulumiValueContent;
                match &value.content {
                    PulumiValueContent::Number(f) => {
                        match f {
                            $($value => Ok($enum_name::$variant)),*,
                            _ => bail!("Invalid number enum value: {}", f),
                        }
                    }
                    _ => bail!("Expected Number, got {:?}", value.content),
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{FromPulumiValue, PulumiValueContent, ToPulumiValue};
    use futures::executor::block_on;

    #[derive(Debug, PartialEq, Clone)]
    enum Color {
        Red,
        Blue,
    }

    pulumi_value_string_enum! {
        enum Color {
            Red => "red",
            Blue => "blue",
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    enum Size {
        Small,
        Large,
    }

    pulumi_value_integer_enum! {
        enum Size {
            Small => 1,
            Large => 2,
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    enum Brightness {
        Dim,
        Full,
    }

    pulumi_value_number_enum! {
        enum Brightness {
            Dim => 0.1,
            Full => 1.0,
        }
    }

    #[test]
    fn string_enum_round_trip() {
        let value = block_on(Color::Blue.to_pulumi_value());
        assert_eq!(
            value.content,
            PulumiValueContent::String("blue".to_string())
        );

        let parsed = Color::from_pulumi_value(&value).unwrap();
        assert_eq!(parsed, Color::Blue);
    }

    #[test]
    fn integer_enum_round_trip() {
        let value = block_on(Size::Large.to_pulumi_value());
        assert_eq!(value.content, PulumiValueContent::Integer(2));

        let parsed = Size::from_pulumi_value(&value).unwrap();
        assert_eq!(parsed, Size::Large);
    }

    #[test]
    fn number_enum_round_trip() {
        let value = block_on(Brightness::Dim.to_pulumi_value());
        assert_eq!(value.content, PulumiValueContent::Number(0.1));

        let parsed = Brightness::from_pulumi_value(&value).unwrap();
        assert_eq!(parsed, Brightness::Dim);
    }

    #[test]
    fn string_enum_invalid_value() {
        let value = block_on("green".to_pulumi_value());
        let err = Color::from_pulumi_value(&value).unwrap_err();
        assert!(err.to_string().contains("Invalid string enum value: green"));
    }

    #[test]
    fn integer_enum_invalid_type() {
        let value = block_on("not-int".to_pulumi_value());
        let err = Size::from_pulumi_value(&value).unwrap_err();
        assert!(err.to_string().contains("Expected Integer"));
    }

    #[test]
    fn number_enum_invalid_value() {
        let value = block_on(3.0f64.to_pulumi_value());
        let err = Brightness::from_pulumi_value(&value).unwrap_err();
        assert!(err.to_string().contains("Invalid number enum value: 3"));
    }
}
