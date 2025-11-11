mod config;
mod engine;
mod model;

pub use crate::engine::ConfigValue;
pub use crate::config::Config;
pub use crate::engine::Engine;
pub use crate::model::FunctionName;
use futures::future::{BoxFuture, Shared};
use futures::FutureExt;
use pulumi_gestalt_domain::{NodeValue, ResourceFields};
use std::sync::Arc;

pub type RawOutput = Output<NodeValue>;

impl RawOutput {
    pub fn from_node_value(value: NodeValue) -> Self {
        let f = async move { value };
        Self {
            value: f.boxed().shared(),
        }
    }
}

pub type RegisterResourceOutput = Output<Arc<ResourceFields>>;

#[derive(Clone)]
struct Output<T> {
    value: Shared<BoxFuture<'static, T>>,
}

impl<T: Clone> Output<T> {
    pub fn from_future<F>(future: F) -> Output<T>
    where
        F: Future<Output = T> + Send + 'static,
    {
        Self {
            value: future.boxed().shared(),
        }
    }

    // Used for mappings to ensure they will be invoked (even if the result is not needed)
    pub fn invoke_void(self) -> impl Future<Output = ()> {
        async move {
            self.value.await;
        }
    }
}
