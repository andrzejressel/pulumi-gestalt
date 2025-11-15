#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScheduleTargetEcsParametersCapacityProviderStrategy {
    /// How many tasks, at a minimum, to run on the specified capacity provider. Only one capacity provider in a capacity provider strategy can have a base defined. Ranges from `0` (default) to `100000`.
    #[builder(into)]
    #[serde(rename = "base")]
    pub r#base: Option<i32>,
    /// Short name of the capacity provider.
    #[builder(into)]
    #[serde(rename = "capacityProvider")]
    pub r#capacity_provider: String,
    /// Designates the relative percentage of the total number of tasks launched that should use the specified capacity provider. The weight value is taken into consideration after the base value, if defined, is satisfied. Ranges from from `0` to `1000`.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Option<i32>,
}
