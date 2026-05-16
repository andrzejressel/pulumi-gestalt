#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareNodePoolNodePoolAutoscaling {
    /// Maximum number of replicas in the NodePool.
    #[builder(into)]
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: i32,
    /// Minimum number of replicas in the NodePool.
    #[builder(into)]
    #[serde(rename = "minReplicas")]
    pub r#min_replicas: i32,
}
