use std::collections::HashSet;
use NodeValue::Nothing;
use futures::FutureExt;
use futures::future::{BoxFuture, Shared};
use std::future::Future;
use std::ops::Deref;
use std::sync::Arc;
use crate::output::NodeValue::Exists;

#[derive(Debug, PartialEq)]
pub(crate) enum NodeValue<T> {
    Nothing,
    Exists(Arc<T>),
}

impl<T> From<T> for NodeValue<T> {
    fn from(value: T) -> Self {
        Exists(Arc::new(value))
    }
}

impl<T> Clone for NodeValue<T> {
    fn clone(&self) -> Self {
        match self {
            Nothing => Nothing,
            Exists(value) => Exists(value.clone()),
        }
    }
}

impl<T> NodeValue<T> {
    fn map<U>(&self, f: fn(&T) -> U) -> NodeValue<U> {
        match self {
            Nothing => Nothing,
            Exists(value) => Exists(Arc::new(f(value.deref()))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Node<T> {
    pub(crate) node_value: NodeValue<T>,
    pub(crate) secret: bool,
    pub(crate) dependencies: HashSet<String>,
}

impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            node_value: self.node_value.clone(),
            secret: self.secret,
            dependencies: self.dependencies.clone(),
        }
    }
}

impl<T> Node<T> {
    pub(crate) fn map<U>(&self, f: fn(&T) -> U) -> Node<U> {
        Node {
            node_value: self.node_value.map(f),
            secret: self.secret,
            dependencies: self.dependencies.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Output<T> {
    future: Shared<BoxFuture<'static, Arc<Node<T>>>>,
}

impl<T: Send + Sync + 'static> Output<T> {
    pub fn new_nothing() -> Self {
        let value = Arc::new(Node {
            node_value: Nothing,
            secret: false,
            dependencies: HashSet::new(),
        });
        Self {
            future: futures::future::ready(value).boxed().shared(),
        }
    }

    pub fn new(value: T) -> Self {
        let value = Arc::new(Node {
            node_value: value.into(),
            secret: false,
            dependencies: HashSet::new(),
        });
        Self {
            future: futures::future::ready(value).boxed().shared(),
        }
    }

    pub fn new_secret(value: T) -> Self {
        let value = Arc::new(Node {
            node_value: value.into(),
            secret: true,
            dependencies: HashSet::new(),
        });
        Self {
            future: futures::future::ready(value).boxed().shared(),
        }
    }

    pub fn secret(self) -> Self {
        Self {
            future: self
                .future
                .map(|node| {
                    Arc::new(Node {
                        node_value: node.node_value.clone(),
                        secret: true,
                        dependencies: node.dependencies.clone(),
                    })
                })
                .boxed()
                .shared(),
        }
    }

    pub fn unsecret(self) -> Self {
        Self {
            future: self
                .future
                .map(|node| {
                    Arc::new(Node {
                        node_value: node.node_value.clone(),
                        secret: false,
                        dependencies: node.dependencies.clone(),
                    })
                })
                .boxed()
                .shared(),
        }
    }

    fn from_future(future: impl Future<Output = Arc<Node<T>>> + Send + 'static) -> Self {
        Self {
            future: future.boxed().shared(),
        }
    }

    pub fn map<T2: Sync + Send + 'static>(&self, f: fn(&T) -> T2) -> Output<T2> {
        Output::from_future(self.future.clone().map(move |value| Arc::new(value.map(f))))
    }

    pub fn and_then<T2: Sync + Send + 'static>(&self, f: fn(&T) -> Output<T2>) -> Output<T2> {
        Output::from_future(self.future.clone().then(move |node| async move {
            match &node.node_value {
                Nothing => Arc::new(Node {
                    node_value: Nothing,
                    secret: node.secret,
                    dependencies: node.dependencies.clone(),
                }),
                Exists(value) => {
                    let next_output = f(value.as_ref());
                    let next_node = next_output.future.await;

                    let mut dependencies = node.dependencies.clone();
                    dependencies.extend(next_node.dependencies.iter().cloned());

                    Arc::new(Node {
                        node_value: next_node.node_value.clone(),
                        secret: node.secret || next_node.secret,
                        dependencies,
                    })
                }
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn test_create_nonsecret_output() {
        let output = Output::new(42);
        let value = futures::executor::block_on(output.future.clone());
        assert_eq!(
            value.deref(),
            &Node {
                node_value: Exists(42.into()),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_create_secret_output() {
        let output = Output::new_secret(42);
        let value = futures::executor::block_on(output.future.clone());
        assert_eq!(
            value.deref(),
            &Node {
                node_value: Exists(42.into()),
                secret: true,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_mapping() {
        let output = Output::new(42);
        let mapped_output = output.map(|x| x * 2);
        let value = futures::executor::block_on(mapped_output.future.clone());
        assert_eq!(
            value.deref(),
            &Node {
                node_value: Exists(84.into()),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_secret_mapping() {
        let output = Output::new_secret(42);
        let mapped_output = output.map(|x| x * 2);
        let value = futures::executor::block_on(mapped_output.future.clone());
        assert_eq!(
            value.deref(),
            &Node {
                node_value: Exists(84.into()),
                secret: true,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_nothing_mapping() {
        let output = Output::new_nothing();
        let mapped_output = output.map(|x| x * 2);
        let value = futures::executor::block_on(mapped_output.future.clone());
        assert_eq!(
            value.deref(),
            &Node {
                node_value: Nothing,
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_secret_nothing_mapping() {
        let output = Output::new_nothing().secret();
        let mapped_output = output.map(|x| x * 2);
        let value = futures::executor::block_on(mapped_output.future.clone());
        assert_eq!(
            value.deref(),
            &Node {
                node_value: Nothing,
                secret: true,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_secret() {
        let output = Output::new(42).secret();
        let value = futures::executor::block_on(output.future.clone());
        assert_eq!(
            value.deref(),
            &Node {
                node_value: 42.into(),
                secret: true,
                dependencies: HashSet::new(),
            }
        )
    }

    #[test]
    fn test_unsecret() {
        let output = Output::new_secret(42).unsecret();
        let value = futures::executor::block_on(output.future.clone());
        assert_eq!(
            value.deref(),
            &Node {
                node_value: 42.into(),
                secret: false,
                dependencies: HashSet::new(),
            }
        )
    }

    #[test]
    fn test_and_then() {
        let output = Output::new(21);
        let result = output.and_then(|x| Output::new(x * 2));
        let value = futures::executor::block_on(result.future.clone());
        assert_eq!(
            value.deref(),
            &Node {
                node_value: 42.into(),
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_and_then_nothing() {
        let output = Output::new_nothing();
        let result = output.and_then(|x: &i32| Output::new(x * 2));
        let value = futures::executor::block_on(result.future.clone());
        assert_eq!(
            value.deref(),
            &Node {
                node_value: Nothing,
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_and_then_to_nothing() {
        let output = Output::new(21);
        let result = output.and_then(|_| Output::<i32>::new_nothing());
        let value = futures::executor::block_on(result.future.clone());
        assert_eq!(
            value.deref(),
            &Node {
                node_value: Nothing,
                secret: false,
                dependencies: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_and_then_secret_propagation() {
        // Source secret, next non-secret
        let out1 = Output::new_secret(21);
        let res1 = out1.and_then(|x| Output::new(x * 2));
        assert!(futures::executor::block_on(res1.future.clone()).secret);

        // Source non-secret, next secret
        let out2 = Output::new(21);
        let res2 = out2.and_then(|x| Output::new_secret(x * 2));
        assert!(futures::executor::block_on(res2.future.clone()).secret);

        // Both secret
        let out3 = Output::new_secret(21);
        let res3 = out3.and_then(|x| Output::new_secret(x * 2));
        assert!(futures::executor::block_on(res3.future.clone()).secret);

        // Neither secret
        let out4 = Output::new(21);
        let res4 = out4.and_then(|x| Output::new(x * 2));
        assert!(!futures::executor::block_on(res4.future.clone()).secret);
    }

    #[test]
    fn test_and_then_dependencies_propagation() {
        let mut deps1 = HashSet::new();
        deps1.insert("dep1".to_string());
        
        let out1 = Output::from_future(futures::future::ready(Arc::new(Node {
            node_value: 21.into(),
            secret: false,
            dependencies: deps1,
        })));

        let result = out1.and_then(|x| {
            let mut deps2 = HashSet::new();
            deps2.insert("dep2".to_string());
            Output::from_future(futures::future::ready(Arc::new(Node {
                node_value: (x * 2).into(),
                secret: false,
                dependencies: deps2,
            })))
        });

        let value = futures::executor::block_on(result.future.clone());
        let expected_deps: HashSet<_> = ["dep1", "dep2"].iter().map(|s| s.to_string()).collect();
        assert_eq!(value.dependencies, expected_deps);
    }
}
