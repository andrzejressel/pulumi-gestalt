#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskSetCapacityProviderStrategy {
    /// The number of tasks, at a minimum, to run on the specified capacity provider. Only one capacity provider in a capacity provider strategy can have a base defined.
    #[builder(into)]
    #[serde(rename = "base")]
    pub r#base: Option<i32>,
    /// The short name or full Amazon Resource Name (ARN) of the capacity provider.
    #[builder(into)]
    #[serde(rename = "capacityProvider")]
    pub r#capacity_provider: String,
    /// The relative percentage of the total number of launched tasks that should use the specified capacity provider.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: i32,
}
