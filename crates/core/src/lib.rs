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

    pub fn secret(&self) -> Self {
        let value = self.value.clone();
        Self::from_future(async move {
            match value.await {
                NodeValue::Nothing => NodeValue::Nothing,
                NodeValue::Exists(mut existing) => {
                    existing.secret = true;
                    NodeValue::Exists(existing)
                }
            }
        })
    }

    pub fn unsecret(&self) -> Self {
        let value = self.value.clone();
        Self::from_future(async move {
            match value.await {
                NodeValue::Nothing => NodeValue::Nothing,
                NodeValue::Exists(mut existing) => {
                    existing.secret = false;
                    NodeValue::Exists(existing)
                }
            }
        })
    }
}

#[derive(Clone)]
pub struct RegisterResourceOutput {
    pub(crate) fields: Output<Arc<ResourceFields>>,
    pub(crate) urn: RawOutput,
    pub(crate) id: RawOutput,
    /// Pulumi Provider ID is the combination of URN and ID. It is used when creating a resource.
    pub(crate) provider_id: RawOutput,
}

impl RegisterResourceOutput {
    pub fn get_urn(&self) -> RawOutput {
        self.urn.clone()
    }

    pub fn get_id(&self) -> RawOutput {
        self.id.clone()
    }

    pub fn get_provider_id(&self) -> RawOutput {
        self.provider_id.clone()
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

#[cfg(test)]
mod tests {
    use super::RawOutput;
    use pulumi_gestalt_domain::{ExistingNodeValue, NodeValue};
    use serde_json::json;

    #[tokio::test]
    async fn secret_sets_secret_flag_to_true() {
        let output = RawOutput::from_node_value(NodeValue::exists(json!(42), false));

        let result = output.secret().value.await;

        assert_eq!(
            result,
            NodeValue::Exists(ExistingNodeValue {
                value: json!(42),
                secret: true,
            })
        );
    }

    #[tokio::test]
    async fn unsecret_sets_secret_flag_to_false() {
        let output = RawOutput::from_node_value(NodeValue::exists(json!("x"), true));

        let result = output.unsecret().value.await;

        assert_eq!(
            result,
            NodeValue::Exists(ExistingNodeValue {
                value: json!("x"),
                secret: false,
            })
        );
    }

    #[tokio::test]
    async fn secret_and_unsecret_leave_nothing_unchanged() {
        let output = RawOutput::from_node_value(NodeValue::Nothing);

        assert_eq!(output.secret().value.await, NodeValue::Nothing);
        assert_eq!(output.unsecret().value.await, NodeValue::Nothing);
    }
}
