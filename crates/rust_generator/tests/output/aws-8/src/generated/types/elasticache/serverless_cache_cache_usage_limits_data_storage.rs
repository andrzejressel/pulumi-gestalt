#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServerlessCacheCacheUsageLimitsDataStorage {
    /// The upper limit for data storage the cache is set to use. Must be between 1 and 5,000.
    #[builder(into)]
    #[serde(rename = "maximum")]
    pub r#maximum: Option<i32>,
    /// The lower limit for data storage the cache is set to use. Must be between 1 and 5,000.
    #[builder(into)]
    #[serde(rename = "minimum")]
    pub r#minimum: Option<i32>,
    /// The unit that the storage is measured in, in GB.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: String,
}
