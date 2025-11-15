#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketLifecycleConfigurationRule {
    /// Configuration block containing settings for abort incomplete multipart upload.
    #[builder(into)]
    #[serde(rename = "abortIncompleteMultipartUpload")]
    pub r#abort_incomplete_multipart_upload: Option<Box<super::super::types::s3control::BucketLifecycleConfigurationRuleAbortIncompleteMultipartUpload>>,
    /// Configuration block containing settings for expiration of objects.
    #[builder(into)]
    #[serde(rename = "expiration")]
    pub r#expiration: Option<Box<super::super::types::s3control::BucketLifecycleConfigurationRuleExpiration>>,
    /// Configuration block containing settings for filtering.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Option<Box<super::super::types::s3control::BucketLifecycleConfigurationRuleFilter>>,
    /// Unique identifier for the rule.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Status of the rule. Valid values: `Enabled` and `Disabled`. Defaults to `Enabled`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
