#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodePoolUpgradeSettingBlueGreenSettingStandardRolloutPolicy {
    /// Number of blue nodes to drain in a batch.
    #[builder(into)]
    #[serde(rename = "batchNodeCount")]
    pub r#batch_node_count: i32,
    /// Percentage of the blue pool nodes to drain in a batch.
    #[builder(into)]
    #[serde(rename = "batchPercentage")]
    pub r#batch_percentage: f64,
    /// Soak time after each batch gets drained.
    #[builder(into)]
    #[serde(rename = "batchSoakDuration")]
    pub r#batch_soak_duration: String,
}
