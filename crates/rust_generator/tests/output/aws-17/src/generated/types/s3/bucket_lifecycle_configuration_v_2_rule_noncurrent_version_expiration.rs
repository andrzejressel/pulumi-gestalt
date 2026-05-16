#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketLifecycleConfigurationV2RuleNoncurrentVersionExpiration {
    /// Number of noncurrent versions Amazon S3 will retain. Must be a non-zero positive integer.
    #[builder(into)]
    #[serde(rename = "newerNoncurrentVersions")]
    pub r#newer_noncurrent_versions: Option<String>,
    /// Number of days an object is noncurrent before Amazon S3 can perform the associated action. Must be a positive integer.
    #[builder(into)]
    #[serde(rename = "noncurrentDays")]
    pub r#noncurrent_days: Option<i32>,
}
