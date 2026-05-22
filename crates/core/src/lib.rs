mod config;
mod engine;
mod model;

pub use config::Config;
pub use engine::ConfigValue;
pub use engine::Engine;
use futures::FutureExt;
use futures::future::{BoxFuture, Shared};
pub use model::FunctionName;
use pulumi_gestalt_domain::ResourceFields;
use pulumi_gestalt_model::PulumiValue;
use std::future::Future;
use std::sync::Arc;

pub type RawOutput = Output<PulumiValue>;

impl RawOutput {
    pub(crate) fn from_pulumi_value(value: PulumiValue) -> Self {
        let f = async move { value };
        Self {
            value: f.boxed().shared(),
        }
    }

    pub fn secret(&self) -> Self {
        let value = self.value.clone();
        Self::from_future(async move {
            let mut result = value.await;
            result.secret = true;
            result
        })
    }

    pub fn unsecret(&self) -> Self {
        let value = self.value.clone();
        Self::from_future(async move {
            let mut result = value.await;
            result.secret = false;
            result
        })
    }

    pub fn from_future_pulumi_value<F>(future: F) -> Self
    where
        F: Future<Output = PulumiValue> + Send + 'static,
    {
        Self::from_future(future)
    }

    pub async fn resolve_pulumi_value(&self) -> PulumiValue {
        self.value.clone().await
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
    pub fn from_future<F>(future: F) -> Output<T>
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
    use pulumi_gestalt_model::{PulumiValue, PulumiValueContent};

    #[tokio::test]
    async fn secret_sets_secret_flag_to_true() {
        let output = RawOutput::from_pulumi_value(PulumiValue {
            content: PulumiValueContent::Integer(42),
            secret: false,
            dependencies: Default::default(),
        });

        let result = output.secret().value.await;

        assert_eq!(
            result,
            PulumiValue {
                content: PulumiValueContent::Integer(42),
                secret: true,
                dependencies: Default::default(),
            }
        );
    }

    #[tokio::test]
    async fn unsecret_sets_secret_flag_to_false() {
        let output = RawOutput::from_pulumi_value(PulumiValue {
            content: PulumiValueContent::String("x".to_string()),
            secret: true,
            dependencies: Default::default(),
        });

        let result = output.unsecret().value.await;

        assert_eq!(
            result,
            PulumiValue {
                content: PulumiValueContent::String("x".to_string()),
                secret: false,
                dependencies: Default::default(),
            }
        );
    }

    #[tokio::test]
    async fn secret_and_unsecret_leave_nothing_unchanged() {
        let output = RawOutput::from_pulumi_value(PulumiValue::nothing());

        assert_eq!(
            output.secret().value.await,
            PulumiValue {
                content: PulumiValueContent::Nothing,
                secret: true,
                dependencies: Default::default(),
            }
        );
        assert_eq!(
            output.unsecret().value.await,
            PulumiValue {
                content: PulumiValueContent::Nothing,
                secret: false,
                dependencies: Default::default(),
            }
        );
    }
}
