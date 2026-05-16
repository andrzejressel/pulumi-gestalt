#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NodeGroupUpdateConfig {
    /// Desired max number of unavailable worker nodes during node group update.
    #[builder(into)]
    #[serde(rename = "maxUnavailable")]
    pub r#max_unavailable: Option<i32>,
    /// Desired max percentage of unavailable worker nodes during node group update.
    #[builder(into)]
    #[serde(rename = "maxUnavailablePercentage")]
    pub r#max_unavailable_percentage: Option<i32>,
}
