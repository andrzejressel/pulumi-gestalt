#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPoolFixedScale {
    /// The timeout for resize operations.
    #[builder(into)]
    #[serde(rename = "resizeTimeout")]
    pub r#resize_timeout: String,
    /// The number of nodes in the Batch pool.
    #[builder(into)]
    #[serde(rename = "targetDedicatedNodes")]
    pub r#target_dedicated_nodes: i32,
    /// The number of low priority nodes in the Batch pool.
    #[builder(into)]
    #[serde(rename = "targetLowPriorityNodes")]
    pub r#target_low_priority_nodes: i32,
}
