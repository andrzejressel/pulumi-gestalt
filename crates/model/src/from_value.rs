use crate::output::{Node, NodeValue, Output};
use crate::{PulumiValue, PulumiValueContent};

use rootcause::prelude::ResultExt;
use rootcause::{Result, bail};
use std::collections::BTreeMap;
use std::boxed::Box;
use std::sync::Arc;

pub trait FromPulumiValue {
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self>
    where
        Self: Sized;
}

impl FromPulumiValue for String {
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self> {
        match value.content {
            PulumiValueContent::String(ref s) => Ok(s.clone()),
            _ => bail!("Expected String, got {:?}", value.content),
        }
    }
}

impl FromPulumiValue for i32 {
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self> {
        match value.content {
            PulumiValueContent::Integer(i) => Ok(i),
            _ => bail!("Expected Integer, got {:?}", value.content),
        }
    }
}

impl FromPulumiValue for f64 {
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self> {
        match value.content {
            PulumiValueContent::Number(f) => Ok(f),
            _ => bail!("Expected Number, got {:?}", value.content),
        }
    }
}

impl FromPulumiValue for bool {
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self> {
        match value.content {
            PulumiValueContent::Boolean(b) => Ok(b),
            _ => bail!("Expected Boolean, got {:?}", value.content),
        }
    }
}

impl<T> FromPulumiValue for Option<T>
where
    T: FromPulumiValue,
{
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self> {
        match value.content {
            PulumiValueContent::None => Ok(None),
            _ => Ok(Some(T::from_pulumi_value(value).context_with(
                move || {
                    format!(
                        "Failed to convert PulumiValue [{:?}] to Option<{}>",
                        value,
                        std::any::type_name::<T>()
                    )
                },
            )?)),
        }
    }
}

impl<T> FromPulumiValue for Vec<T>
where
    T: FromPulumiValue,
{
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self> {
        match value.content {
            PulumiValueContent::Array(ref arr) => arr
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    Ok(T::from_pulumi_value(v).context_with(move || {
                        format!(
                            "Failed to convert PulumiValue at index {} to {}",
                            i,
                            std::any::type_name::<T>()
                        )
                    })?)
                })
                .collect(),
            _ => bail!("Expected Array, got {:?}", value.content),
        }
    }
}

impl<T> FromPulumiValue for Box<T>
where
    T: FromPulumiValue,
{
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self> {
        Ok(Box::new(T::from_pulumi_value(value).context_with(move || {
            format!(
                "Failed to convert PulumiValue [{:?}] to Box<{}>",
                value,
                std::any::type_name::<T>()
            )
        })?))
    }
}

impl<T> FromPulumiValue for BTreeMap<String, T>
where
    T: FromPulumiValue,
{
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self> {
        match value.content {
            PulumiValueContent::Object(ref obj) => obj
                .iter()
                .map(|(k, v)| {
                    Ok((
                        k.clone(),
                        T::from_pulumi_value(v).context_with(move || {
                            format!(
                                "Failed to convert PulumiValue at key '{}' to {}",
                                k,
                                std::any::type_name::<T>()
                            )
                        })?,
                    ))
                })
                .collect(),
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

impl<T> FromPulumiValue for Output<T>
where
    T: FromPulumiValue + Send + Sync + 'static,
{
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self> {
        let node_value = match value.content {
            PulumiValueContent::Nothing => NodeValue::Nothing,
            _ => NodeValue::Exists(Arc::new(T::from_pulumi_value(value).context_with(
                move || {
                    format!(
                        "Failed to convert PulumiValue to Output<{}>",
                        std::any::type_name::<T>()
                    )
                },
            )?)),
        };

        Ok(Self::from_node(Node {
            node_value,
            secret: value.secret,
            dependencies: value.dependencies.clone(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_option_string_none() {
        let value = PulumiValue {
            content: PulumiValueContent::None,
            secret: false,
            dependencies: HashSet::new(),
        };

        let result: Result<Option<String>> = Option::<String>::from_pulumi_value(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_option_string_some() {
        let value = PulumiValue {
            content: PulumiValueContent::String("hello".to_string()),
            secret: false,
            dependencies: HashSet::new(),
        };

        let result: Result<Option<String>> = Option::<String>::from_pulumi_value(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("hello".to_string()));
    }

    #[test]
    fn test_option_string_error() {
        let value = PulumiValue {
            content: PulumiValueContent::Integer(42),
            secret: false,
            dependencies: HashSet::new(),
        };

        let result: Result<Option<String>> = Option::<String>::from_pulumi_value(&value);
        assert!(result.is_err());
    }

    #[test]
    fn test_vec_i32() {
        let value = PulumiValue {
            content: PulumiValueContent::Array(vec![
                PulumiValue {
                    content: PulumiValueContent::Integer(1),
                    secret: false,
                    dependencies: HashSet::new(),
                },
                PulumiValue {
                    content: PulumiValueContent::Integer(2),
                    secret: false,
                    dependencies: HashSet::new(),
                },
            ]),
            secret: false,
            dependencies: HashSet::new(),
        };

        let result: Result<Vec<i32>> = Vec::<i32>::from_pulumi_value(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2]);
    }

    #[test]
    fn test_btreemap_string() {
        let value = PulumiValue {
            content: PulumiValueContent::Object(vec![(
                "key".to_string(),
                PulumiValue {
                    content: PulumiValueContent::String("value".to_string()),
                    secret: false,
                    dependencies: HashSet::new(),
                },
            )]),
            secret: false,
            dependencies: HashSet::new(),
        };

        let result: Result<BTreeMap<String, String>> =
            BTreeMap::<String, String>::from_pulumi_value(&value);
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.get("key").unwrap(), "value");
    }

    #[test]
    fn test_box_string() {
        let value = PulumiValue {
            content: PulumiValueContent::String("hello".to_string()),
            secret: false,
            dependencies: HashSet::new(),
        };

        let result: Result<Box<String>> = Box::<String>::from_pulumi_value(&value);
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), "hello".to_string());
    }

    #[test]
    fn test_box_error_context() {
        let value = PulumiValue {
            content: PulumiValueContent::Integer(42),
            secret: false,
            dependencies: HashSet::new(),
        };

        let result: Result<Box<String>> = Box::<String>::from_pulumi_value(&value);
        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("Failed to convert PulumiValue")
        );
    }

    #[test]
    fn test_vec_error() {
        let value = PulumiValue {
            content: PulumiValueContent::Array(vec![PulumiValue {
                content: PulumiValueContent::String("not an integer".to_string()),
                secret: false,
                dependencies: HashSet::new(),
            }]),
            secret: false,
            dependencies: HashSet::new(),
        };

        let result: Result<Vec<i32>> = Vec::<i32>::from_pulumi_value(&value);
        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("Failed to convert PulumiValue at index 0 to i32")
        );
    }

    #[test]
    fn test_output_string_secret() {
        let mut dependencies = HashSet::new();
        dependencies.insert("dep1".to_string());

        let value = PulumiValue {
            content: PulumiValueContent::String("secret message".to_string()),
            secret: true,
            dependencies,
        };

        let result: Result<Output<String>> = Output::<String>::from_pulumi_value(&value);
        assert!(result.is_ok());
        let output = result.unwrap();
        let node = futures::executor::block_on(output.future);

        assert!(node.secret);
        assert!(node.dependencies.contains("dep1"));
        match &node.node_value {
            NodeValue::Exists(v) => assert_eq!(v.as_ref(), "secret message"),
            _ => panic!("Expected Exists"),
        }
    }

    #[test]
    fn test_output_nothing() {
        let value = PulumiValue {
            content: PulumiValueContent::Nothing,
            secret: false,
            dependencies: HashSet::new(),
        };

        let result: Result<Output<String>> = Output::<String>::from_pulumi_value(&value);
        assert!(result.is_ok());
        let output = result.unwrap();
        let node = futures::executor::block_on(output.future);

        assert!(!node.secret);
        match &node.node_value {
            NodeValue::Nothing => {}
            _ => panic!("Expected Nothing"),
        }
    }

    #[test]
    fn test_output_option_none() {
        let value = PulumiValue {
            content: PulumiValueContent::None,
            secret: false,
            dependencies: HashSet::new(),
        };

        let result: Result<Output<Option<String>>> =
            Output::<Option<String>>::from_pulumi_value(&value);
        assert!(result.is_ok());
        let output = result.unwrap();
        let node = futures::executor::block_on(output.future);

        match &node.node_value {
            NodeValue::Exists(v) => assert_eq!(v.as_ref(), &None),
            _ => panic!("Expected Exists(None)"),
        }
    }

    #[test]
    fn test_output_error_context() {
        let value = PulumiValue {
            content: PulumiValueContent::Integer(42),
            secret: false,
            dependencies: HashSet::new(),
        };

        let result: Result<Output<String>> = Output::<String>::from_pulumi_value(&value);
        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("Failed to convert PulumiValue to Output<alloc::string::String>")
        );
    }
}
