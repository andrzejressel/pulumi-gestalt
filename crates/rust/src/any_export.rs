use crate::{Context, Output, PulumiAny, ToPulumiAny};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait IntoOutputAny {
    fn as_output(&self, ctx: &Context) -> Output<PulumiAny>;
}

impl<T: Serialize> IntoOutputAny for T {
    fn as_output(&self, ctx: &Context) -> Output<PulumiAny> {
        ctx.new_output(&self.to_pulumi_any())
    }
}

impl<T: Serialize + DeserializeOwned> IntoOutputAny for Output<T> {
    fn as_output(&self, _: &Context) -> Output<PulumiAny> {
        self.clone().to_pulumi_any()
    }
}
