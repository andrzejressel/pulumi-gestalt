#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNodeGroupScalingConfig {
    /// Desired number of worker nodes.
    #[builder(into)]
    #[serde(rename = "desiredSize")]
    pub r#desired_size: i32,
    /// Maximum number of worker nodes.
    #[builder(into)]
    #[serde(rename = "maxSize")]
    pub r#max_size: i32,
    /// Minimum number of worker nodes.
    #[builder(into)]
    #[serde(rename = "minSize")]
    pub r#min_size: i32,
}
