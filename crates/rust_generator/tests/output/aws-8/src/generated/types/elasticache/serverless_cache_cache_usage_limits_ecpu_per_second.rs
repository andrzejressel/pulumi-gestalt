#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServerlessCacheCacheUsageLimitsEcpuPerSecond {
    /// The maximum number of ECPUs the cache can consume per second. Must be between 1,000 and 15,000,000.
    #[builder(into)]
    #[serde(rename = "maximum")]
    pub r#maximum: Option<i32>,
    /// The minimum number of ECPUs the cache can consume per second. Must be between 1,000 and 15,000,000.
    #[builder(into)]
    #[serde(rename = "minimum")]
    pub r#minimum: Option<i32>,
}
