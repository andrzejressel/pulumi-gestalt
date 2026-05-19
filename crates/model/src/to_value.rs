use crate::output::NodeValue;
use crate::{Output, PulumiValue, PulumiValueContent};
use futures::FutureExt;
use futures::future::{BoxFuture, Shared};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::future::Future;

pub type ToPulumiObjectFieldFuture<'a> = futures::future::BoxFuture<'a, (String, PulumiValue)>;

#[derive(Clone)]
pub enum PulumiValueMiddleware {
    Ready(PulumiValue),
    Deferred(Shared<BoxFuture<'static, PulumiValue>>),
}

impl PulumiValueMiddleware {
    pub fn ready(value: PulumiValue) -> Self {
        Self::Ready(value)
    }

    pub fn deferred(future: impl Future<Output = PulumiValue> + Send + 'static) -> Self {
        Self::Deferred(future.boxed().shared())
    }

    fn into_future(self) -> BoxFuture<'static, PulumiValue> {
        match self {
            Self::Ready(value) => futures::future::ready(value).boxed(),
            Self::Deferred(future) => future.boxed(),
        }
    }
}

pub fn to_pulumi_object_field<'a, T>(
    name: &'static str,
    value: &'a T,
) -> ToPulumiObjectFieldFuture<'a>
where
    T: ToPulumiValue + Sync + ?Sized + 'a,
{
    use futures::FutureExt;
    let pulumi_value_future = value.to_pulumi_value();

    async move { (name.to_string(), pulumi_value_future.await) }.boxed()
}

pub async fn to_pulumi_object_concurrent<'a>(
    fields: Vec<ToPulumiObjectFieldFuture<'a>>,
) -> PulumiValue {
    let results = futures::future::join_all(fields).await;
    let map: BTreeMap<String, PulumiValue> = results.into_iter().collect();
    ToPulumiValue::to_pulumi_value(&map).await
}

pub fn to_pulumi_value_middleware<T>(value: T) -> PulumiValueMiddleware
where
    T: ToPulumiValue + Sync + Send + 'static,
{
    PulumiValueMiddleware::deferred(async move { value.to_pulumi_value().await })
}

pub fn pulumi_value_middleware(content: PulumiValueContent) -> PulumiValueMiddleware {
    PulumiValueMiddleware::ready(PulumiValue {
        content,
        secret: false,
        dependencies: HashSet::new(),
    })
}

pub fn pulumi_value_middleware_array(values: Vec<PulumiValueMiddleware>) -> PulumiValueMiddleware {
    PulumiValueMiddleware::deferred(async move { values.to_pulumi_value().await })
}

pub fn pulumi_value_middleware_object(
    values: Vec<(String, PulumiValueMiddleware)>,
) -> PulumiValueMiddleware {
    PulumiValueMiddleware::deferred(async move {
        let map: BTreeMap<String, PulumiValueMiddleware> = values.into_iter().collect();
        map.to_pulumi_value().await
    })
}

pub trait ToPulumiValue {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send;
}

impl ToPulumiValue for PulumiValue {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
        futures::future::ready(self.clone())
    }
}

impl ToPulumiValue for String {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
        futures::future::ready(PulumiValue {
            content: PulumiValueContent::String(self.clone()),
            secret: false,
            dependencies: HashSet::new(),
        })
    }
}

impl ToPulumiValue for &str {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
        futures::future::ready(PulumiValue {
            content: PulumiValueContent::String(self.to_string()),
            secret: false,
            dependencies: HashSet::new(),
        })
    }
}

impl ToPulumiValue for i32 {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
        futures::future::ready(PulumiValue {
            content: PulumiValueContent::Integer(*self),
            secret: false,
            dependencies: HashSet::new(),
        })
    }
}

impl ToPulumiValue for f64 {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
        futures::future::ready(PulumiValue {
            content: PulumiValueContent::Number(*self),
            secret: false,
            dependencies: HashSet::new(),
        })
    }
}

impl ToPulumiValue for bool {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
        futures::future::ready(PulumiValue {
            content: PulumiValueContent::Boolean(*self),
            secret: false,
            dependencies: HashSet::new(),
        })
    }
}

impl<T: ToPulumiValue + Sync + Send + 'static> ToPulumiValue for Vec<T> {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
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
    }
}

impl<T: ToPulumiValue + Sync + Send + 'static> ToPulumiValue for Option<T> {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
        let value = self.as_ref().map(ToPulumiValue::to_pulumi_value);
        async move {
            match value {
                Some(future) => future.await,
                None => PulumiValue {
                    content: PulumiValueContent::None,
                    secret: false,
                    dependencies: HashSet::new(),
                },
            }
        }
    }
}

impl<T: ToPulumiValue + Sync + Send + 'static> ToPulumiValue for Box<T> {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
        self.as_ref().to_pulumi_value()
    }
}

impl<T: ToPulumiValue + Sync + Send + 'static> ToPulumiValue for BTreeMap<String, T> {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
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
    }
}

impl<T: ToPulumiValue + Sync + Send + 'static> ToPulumiValue for HashMap<String, T> {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
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
    }
}

impl<T: ToPulumiValue + Sync + Send + 'static> ToPulumiValue for Output<T> {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
        let future = self.future.clone();
        async move {
            let node = future.await;
            match &node.node_value {
                NodeValue::Nothing => PulumiValue {
                    content: PulumiValueContent::Nothing,
                    secret: node.secret,
                    dependencies: node.dependencies.clone(),
                },
                NodeValue::Exists(inner) => {
                    let mut pv = inner.to_pulumi_value().await;
                    pv.secret |= node.secret;
                    pv.dependencies.extend(node.dependencies.iter().cloned());
                    pv
                }
            }
        }
    }
}

impl ToPulumiValue for PulumiValueMiddleware {
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> + Send {
        self.clone().into_future()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::{Node, NodeValue};
    use futures::FutureExt;
    use futures::executor::block_on;
    use std::collections::HashSet;
    use std::sync::Arc;

    #[test]
    fn test_primitive_string() {
        let val = "hello".to_string();
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::String("hello".to_string()),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_primitive_str() {
        let val = "hello";
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::String("hello".to_string()),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_primitive_i32() {
        let val = 42i32;
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Integer(42),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_primitive_f64() {
        let val = 1.23f64;
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Number(1.23),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_primitive_bool() {
        let val = true;
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Boolean(true),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_vec() {
        let val = vec![1i32, 2, 3];
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
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
                    PulumiValue {
                        content: PulumiValueContent::Integer(3),
                        secret: false,
                        dependencies: HashSet::new(),
                    },
                ]),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_btreemap() {
        let mut val = BTreeMap::new();
        val.insert("a".to_string(), 1i32);
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Object(vec![(
                    "a".to_string(),
                    PulumiValue {
                        content: PulumiValueContent::Integer(1),
                        secret: false,
                        dependencies: HashSet::new(),
                    },
                )]),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_option_some_primitive() {
        let val = Some(42i32);
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Integer(42),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_option_none() {
        let val: Option<i32> = None;
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::None,
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_box_primitive() {
        let val = Box::new(42i32);
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Integer(42),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_box_output_propagates_secret_and_deps() {
        let mut deps = HashSet::new();
        deps.insert("dep1".to_string());
        let node = Node {
            node_value: NodeValue::Exists(Arc::new(42i32)),
            secret: true,
            dependencies: deps,
        };
        let val = Box::new(Output::from_future(futures::future::ready(Arc::new(node))));
        let pv = block_on(val.to_pulumi_value());
        let mut expected_deps = HashSet::new();
        expected_deps.insert("dep1".to_string());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Integer(42),
                secret: true,
                dependencies: expected_deps,
            }
        );
    }

    #[test]
    fn test_option_some_output_propagates_secret_and_deps() {
        let mut deps = HashSet::new();
        deps.insert("dep1".to_string());
        let node = Node {
            node_value: NodeValue::Exists(Arc::new(42i32)),
            secret: true,
            dependencies: deps,
        };
        let val = Some(Output::from_future(futures::future::ready(Arc::new(node))));
        let pv = block_on(val.to_pulumi_value());
        let mut expected_deps = HashSet::new();
        expected_deps.insert("dep1".to_string());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Integer(42),
                secret: true,
                dependencies: expected_deps,
            }
        );
    }

    #[test]
    fn test_output_string() {
        let val = Output::new("hello".to_string());
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::String("hello".to_string()),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_output_secret() {
        let val = Output::new_secret(42i32);
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Integer(42),
                secret: true,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_output_nothing() {
        let val: Output<i32> = Output::new_nothing();
        let pv = block_on(val.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Nothing,
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_output_dependencies() {
        let mut deps = HashSet::new();
        deps.insert("dep1".to_string());
        let node = Node {
            node_value: NodeValue::Exists(Arc::new(42i32)),
            secret: false,
            dependencies: deps,
        };
        let val = Output::from_future(futures::future::ready(Arc::new(node)));
        let pv = block_on(val.to_pulumi_value());
        let mut expected_deps = HashSet::new();
        expected_deps.insert("dep1".to_string());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Integer(42),
                secret: false,
                dependencies: expected_deps,
            }
        );
    }

    #[test]
    fn test_nested_output() {
        let inner = Output::new(42i32);
        let outer = Output::new(inner);
        let pv = block_on(outer.to_pulumi_value());
        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Integer(42),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_node_clone_no_t_clone() {
        struct NotClone;
        let node = Node {
            node_value: NodeValue::Exists(Arc::new(NotClone)),
            secret: false,
            dependencies: HashSet::new(),
        };
        let _cloned = node.clone();
    }

    #[test]
    fn test_vec_order_preservation() {
        use futures::channel::oneshot;

        let (tx1, rx1) = oneshot::channel::<Arc<Node<i32>>>();
        let (tx2, rx2) = oneshot::channel::<Arc<Node<i32>>>();

        let out1 = Output::from_future(rx1.map(|r| r.unwrap()));
        let out2 = Output::from_future(rx2.map(|r| r.unwrap()));

        let vec_output = vec![out1, out2];
        let pv_future = vec_output.to_pulumi_value();

        let test_future = async move {
            // Send the second value first
            tx2.send(Arc::new(Node {
                node_value: 2.into(),
                secret: false,
                dependencies: HashSet::new(),
            }))
            .unwrap();

            // Then send the first value
            tx1.send(Arc::new(Node {
                node_value: 1.into(),
                secret: false,
                dependencies: HashSet::new(),
            }))
            .unwrap();

            pv_future.await
        };

        let pv = block_on(test_future);

        assert_eq!(
            pv,
            PulumiValue {
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
            }
        );
    }

    #[test]
    fn test_pulumi_value_middleware_array_propagates_secret_and_dependencies() {
        let mut deps1 = HashSet::new();
        deps1.insert("dep-a".to_string());
        let val1 = Output::from_future(futures::future::ready(Arc::new(Node {
            node_value: NodeValue::Exists(Arc::new(PulumiValue {
                content: PulumiValueContent::Integer(1),
                secret: false,
                dependencies: HashSet::new(),
            })),
            secret: true,
            dependencies: deps1,
        })));

        let mut deps2 = HashSet::new();
        deps2.insert("dep-b".to_string());
        let val2: Output<PulumiValue> =
            Output::from_future(futures::future::ready(Arc::new(Node {
                node_value: NodeValue::Nothing,
                secret: false,
                dependencies: deps2,
            })));

        let out = pulumi_value_middleware_array(vec![
            to_pulumi_value_middleware(val1),
            to_pulumi_value_middleware(val2),
        ]);
        let pv = block_on(out.to_pulumi_value());

        let mut expected_deps = HashSet::new();
        expected_deps.insert("dep-a".to_string());
        expected_deps.insert("dep-b".to_string());

        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Array(vec![
                    PulumiValue {
                        content: PulumiValueContent::Integer(1),
                        secret: true,
                        dependencies: HashSet::new(),
                    },
                    PulumiValue {
                        content: PulumiValueContent::Nothing,
                        secret: false,
                        dependencies: HashSet::new(),
                    },
                ]),
                secret: true,
                dependencies: expected_deps,
            }
        );
    }

    #[test]
    fn test_pulumi_value_middleware_object_sorts_keys_and_last_wins_duplicates() {
        let values = vec![
            (
                "z".to_string(),
                to_pulumi_value_middleware(Output::new(3i32)),
            ),
            (
                "a".to_string(),
                to_pulumi_value_middleware(Output::new(1i32)),
            ),
            (
                "a".to_string(),
                to_pulumi_value_middleware(Output::new(2i32)),
            ),
        ];

        let pv = block_on(pulumi_value_middleware_object(values).to_pulumi_value());

        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Object(vec![
                    (
                        "a".to_string(),
                        PulumiValue {
                            content: PulumiValueContent::Integer(2),
                            secret: false,
                            dependencies: HashSet::new(),
                        },
                    ),
                    (
                        "z".to_string(),
                        PulumiValue {
                            content: PulumiValueContent::Integer(3),
                            secret: false,
                            dependencies: HashSet::new(),
                        },
                    ),
                ]),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_pulumi_value_middleware_object_propagates_secret_and_dependencies() {
        let mut deps1 = HashSet::new();
        deps1.insert("dep-a".to_string());
        let val1 = Output::from_future(futures::future::ready(Arc::new(Node {
            node_value: NodeValue::Exists(Arc::new(PulumiValue {
                content: PulumiValueContent::Integer(1),
                secret: false,
                dependencies: HashSet::new(),
            })),
            secret: true,
            dependencies: deps1,
        })));

        let mut deps2 = HashSet::new();
        deps2.insert("dep-b".to_string());
        let val2: Output<PulumiValue> =
            Output::from_future(futures::future::ready(Arc::new(Node {
                node_value: NodeValue::Nothing,
                secret: false,
                dependencies: deps2,
            })));

        let out = pulumi_value_middleware_object(vec![
            ("b".to_string(), to_pulumi_value_middleware(val2)),
            ("a".to_string(), to_pulumi_value_middleware(val1)),
        ]);
        let pv = block_on(out.to_pulumi_value());

        let mut expected_deps = HashSet::new();
        expected_deps.insert("dep-a".to_string());
        expected_deps.insert("dep-b".to_string());

        assert_eq!(
            pv,
            PulumiValue {
                content: PulumiValueContent::Object(vec![
                    (
                        "a".to_string(),
                        PulumiValue {
                            content: PulumiValueContent::Integer(1),
                            secret: true,
                            dependencies: HashSet::new(),
                        },
                    ),
                    (
                        "b".to_string(),
                        PulumiValue {
                            content: PulumiValueContent::Nothing,
                            secret: false,
                            dependencies: HashSet::new(),
                        },
                    ),
                ]),
                secret: true,
                dependencies: expected_deps,
            }
        );
    }
}
