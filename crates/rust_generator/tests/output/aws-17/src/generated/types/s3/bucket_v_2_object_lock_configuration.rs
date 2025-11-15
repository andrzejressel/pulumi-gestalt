#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketV2ObjectLockConfiguration {
    /// Indicates whether this bucket has an Object Lock configuration enabled. Valid values are `true` or `false`. This argument is not supported in all regions or partitions.
    #[builder(into)]
    #[serde(rename = "objectLockEnabled")]
    pub r#object_lock_enabled: Option<String>,
    /// Object Lock rule in place for this bucket (documented below).
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Option<Vec<super::super::types::s3::BucketV2ObjectLockConfigurationRule>>,
}
