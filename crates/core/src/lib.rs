mod config;
mod engine;
mod model;

pub use config::Config;
pub use engine::ConfigValue;
pub use engine::Engine;
pub use engine::NativeFunctionRequest;
use futures::FutureExt;
use futures::future::{BoxFuture, Shared};
pub use model::FunctionName;
use pulumi_gestalt_domain::{NodeValue, ResourceFields};
use std::sync::Arc;

pub type RawOutput = Output<NodeValue>;

impl RawOutput {
    pub(crate) fn from_node_value(value: NodeValue) -> Self {
        let f = async move { value };
        Self {
            value: f.boxed().shared(),
        }
    }
}

#[derive(Clone)]
pub struct RegisterResourceOutput {
    pub(crate) fields: Output<Arc<ResourceFields>>,
    pub(crate) urn: RawOutput,
}

impl RegisterResourceOutput {
    pub fn get_urn(&self) -> RawOutput {
        self.urn.clone()
    }

    pub(crate) fn invoke_void(self) -> Shared<BoxFuture<'static, ()>> {
        self.fields.invoke_void()
    }
}

#[derive(Clone)]
pub struct Output<T> {
    value: Shared<BoxFuture<'static, T>>,
}

impl<T: Clone + 'static + Send + Sync> Output<T> {
    pub(crate) fn from_future<F>(future: F) -> Output<T>
    where
        F: Future<Output = T> + Send + 'static,
    {
        Self {
            value: future.boxed().shared(),
        }
    }

    // Used for mappings to ensure they will be invoked (even if the result is not needed)
    pub(crate) fn invoke_void(self) -> Shared<BoxFuture<'static, ()>> {
        async move {
            self.value.await;
        }
        .boxed()
        .shared()
    }
}
