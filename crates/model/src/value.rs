use std::collections::{BTreeMap, HashMap, HashSet};
use std::future::Future;
use futures::FutureExt;

#[derive(Clone, Debug, PartialEq)]
pub enum PulumiValueContent {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
    Array(Vec<PulumiValue>),
    Object(Vec<(String, PulumiValue)>),
    Nothing,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PulumiValue {
    pub content: PulumiValueContent,
    pub secret: bool,
    pub dependencies: HashSet<String>,
}

pub trait ToPulumiValue {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Clone + Sync + Send;
}

impl ToPulumiValue for String {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Clone + Sync + Send {
        futures::future::ready(PulumiValue {
            content: PulumiValueContent::String(self.clone()),
            secret: false,
            dependencies: HashSet::new(),
        })
    }
}

impl ToPulumiValue for &str {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Clone + Sync + Send {
        futures::future::ready(PulumiValue {
            content: PulumiValueContent::String(self.to_string()),
            secret: false,
            dependencies: HashSet::new(),
        })
    }
}

impl ToPulumiValue for i64 {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Clone + Sync + Send {
        futures::future::ready(PulumiValue {
            content: PulumiValueContent::Integer(*self),
            secret: false,
            dependencies: HashSet::new(),
        })
    }
}

impl ToPulumiValue for f64 {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Clone + Sync + Send {
        futures::future::ready(PulumiValue {
            content: PulumiValueContent::Number(*self),
            secret: false,
            dependencies: HashSet::new(),
        })
    }
}

impl ToPulumiValue for bool {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Clone + Sync + Send {
        futures::future::ready(PulumiValue {
            content: PulumiValueContent::Boolean(*self),
            secret: false,
            dependencies: HashSet::new(),
        })
    }
}

impl<T: ToPulumiValue + Sync + Send + 'static> ToPulumiValue for Vec<T> {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Clone + Sync + Send {
        let futures: Vec<_> = self.iter().map(|item| item.to_pulumi_value()).collect();
        async move {
            let results = futures::future::join_all(futures).await;
            let mut dependencies = HashSet::new();
            let mut secret = false;
            let mut content = Vec::new();

            for mut val in results {
                dependencies.extend(val.dependencies.drain());
                secret |= val.secret;
                content.push(val);
            }

            PulumiValue {
                content: PulumiValueContent::Array(content),
                secret,
                dependencies,
            }
        }
        .boxed()
        .shared()
    }
}

impl<T: ToPulumiValue + Sync + Send + 'static> ToPulumiValue for HashMap<String, T> {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Clone + Sync + Send {
        let futures: Vec<_> = self
            .iter()
            .map(|(key, value)| {
                let key = key.clone();
                let future = value.to_pulumi_value();
                async move { (key, future.await) }
            })
            .collect();

        async move {
            let results = futures::future::join_all(futures).await;
            let mut dependencies = HashSet::new();
            let mut secret = false;
            let mut content = Vec::new();

            for (key, mut val) in results {
                dependencies.extend(val.dependencies.drain());
                secret |= val.secret;
                content.push((key, val));
            }

            PulumiValue {
                content: PulumiValueContent::Object(content),
                secret,
                dependencies,
            }
        }
        .boxed()
        .shared()
    }
}

impl<T: ToPulumiValue + Sync + Send + 'static> ToPulumiValue for BTreeMap<String, T> {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Clone + Sync + Send {
        let futures: Vec<_> = self
            .iter()
            .map(|(key, value)| {
                let key = key.clone();
                let future = value.to_pulumi_value();
                async move { (key, future.await) }
            })
            .collect();

        async move {
            let results = futures::future::join_all(futures).await;
            let mut dependencies = HashSet::new();
            let mut secret = false;
            let mut content = Vec::new();

            for (key, mut val) in results {
                dependencies.extend(val.dependencies.drain());
                secret |= val.secret;
                content.push((key, val));
            }

            PulumiValue {
                content: PulumiValueContent::Object(content),
                secret,
                dependencies,
            }
        }
        .boxed()
        .shared()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::{Node, NodeValue};
    use futures::executor::block_on;

    #[test]
    fn test_primitive_string() {
        let val = "hello".to_string();
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(pv.content, PulumiValueContent::String("hello".to_string()));
    }

    #[test]
    fn test_primitive_str() {
        let val = "hello";
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(pv.content, PulumiValueContent::String("hello".to_string()));
    }

    #[test]
    fn test_primitive_i64() {
        let val = 42i64;
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(pv.content, PulumiValueContent::Integer(42));
    }

    #[test]
    fn test_primitive_f64() {
        let val = 3.14f64;
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(pv.content, PulumiValueContent::Number(3.14));
    }

    #[test]
    fn test_primitive_bool() {
        let val = true;
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(pv.content, PulumiValueContent::Boolean(true));
    }

    #[test]
    fn test_vec() {
        let val = vec![1i64, 2, 3];
        let pv = block_on(val.to_pulumi_value());
        if let PulumiValueContent::Array(arr) = pv.content {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0].content, PulumiValueContent::Integer(1));
        } else {
            panic!("Expected Array");
        }
    }

    #[test]
    fn test_hashmap() {
        let mut val = HashMap::new();
        val.insert("a".to_string(), 1i64);
        let pv = block_on(val.to_pulumi_value());
        if let PulumiValueContent::Object(obj) = pv.content {
            assert_eq!(obj.len(), 1);
            assert_eq!(obj[0].0, "a");
            assert_eq!(obj[0].1.content, PulumiValueContent::Integer(1));
        } else {
            panic!("Expected Object");
        }
    }

    #[test]
    fn test_node_clone_no_t_clone() {
        struct NotClone;
        let node = Node {
            node_value: NodeValue::Exists(std::sync::Arc::new(NotClone)),
            secret: false,
            dependencies: HashSet::new(),
        };
        let _cloned = node.clone();
    }
}

