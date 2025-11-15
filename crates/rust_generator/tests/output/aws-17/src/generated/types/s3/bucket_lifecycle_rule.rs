#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketLifecycleRule {
    /// Specifies the number of days after initiating a multipart upload when the multipart upload must be completed.
    #[builder(into)]
    #[serde(rename = "abortIncompleteMultipartUploadDays")]
    pub r#abort_incomplete_multipart_upload_days: Option<i32>,
    /// Specifies lifecycle rule status.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Specifies a period in the object's expire (documented below).
    #[builder(into)]
    #[serde(rename = "expiration")]
    pub r#expiration: Option<Box<super::super::types::s3::BucketLifecycleRuleExpiration>>,
    /// Unique identifier for the rule. Must be less than or equal to 255 characters in length.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Specifies when noncurrent object versions expire (documented below).
    #[builder(into)]
    #[serde(rename = "noncurrentVersionExpiration")]
    pub r#noncurrent_version_expiration: Option<Box<super::super::types::s3::BucketLifecycleRuleNoncurrentVersionExpiration>>,
    /// Specifies when noncurrent object versions transitions (documented below).
    /// 
    /// At least one of `abort_incomplete_multipart_upload_days`, `expiration`, `transition`, `noncurrent_version_expiration`, `noncurrent_version_transition` must be specified.
    #[builder(into)]
    #[serde(rename = "noncurrentVersionTransitions")]
    pub r#noncurrent_version_transitions: Option<Vec<super::super::types::s3::BucketLifecycleRuleNoncurrentVersionTransition>>,
    /// Object key prefix identifying one or more objects to which the rule applies.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Specifies object tags key and value.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// Specifies a period in the object's transitions (documented below).
    #[builder(into)]
    #[serde(rename = "transitions")]
    pub r#transitions: Option<Vec<super::super::types::s3::BucketLifecycleRuleTransition>>,
}
