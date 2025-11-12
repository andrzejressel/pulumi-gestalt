mod config;
mod engine;
mod model;

pub use engine::ConfigValue;
pub use config::Config;
pub use engine::Engine;
pub use model::FunctionName;
use futures::future::{BoxFuture, Shared};
use futures::FutureExt;
use pulumi_gestalt_domain::{NodeValue, ResourceFields};
use std::sync::Arc;
pub use engine::NativeFunctionRequest;

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
pub struct Output<T> {
    value: Shared<BoxFuture<'static, T>>,
}

impl<T: Clone + 'static + std::marker::Send + std::marker::Sync> Output<T> {
    pub fn from_future<F>(future: F) -> Output<T>
    where
        F: Future<Output = T> + Send + 'static,
    {
        Self {
            value: future.boxed().shared(),
        }
    }

    // Used for mappings to ensure they will be invoked (even if the result is not needed)
    pub fn invoke_void(self) -> Shared<BoxFuture<'static, ()>> {
        async move {
            self.value.await;
        }.boxed().shared()
    }
}
