use crate::{Output, PulumiValue, ToPulumiValue};

pub trait IntoOutputValue {
    fn as_output(&self) -> Output<PulumiValue>;
}

impl<T: ToPulumiValue + Clone + Send + Sync + 'static> IntoOutputValue for T {
    fn as_output(&self) -> Output<PulumiValue> {
        let value = (*self).clone();
        Output::from_resolved_future(async move {
            let value = value.to_pulumi_value().await;
            crate::ResolvedOutput {
                value: Some(value),
                secret: false,
                dependencies: Default::default(),
            }
        })
    }
}
