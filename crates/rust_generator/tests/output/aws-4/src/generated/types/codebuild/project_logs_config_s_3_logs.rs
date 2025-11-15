#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProjectLogsConfigS3Logs {
    /// Specifies the bucket owner's access for objects that another account uploads to their Amazon S3 bucket. By default, only the account that uploads the objects to the bucket has access to these objects. This property allows you to give the bucket owner access to these objects. Valid values are `NONE`, `READ_ONLY`, and `FULL`. your CodeBuild service role must have the `s3:PutBucketAcl` permission. This permission allows CodeBuild to modify the access control list for the bucket.
    #[builder(into)]
    #[serde(rename = "bucketOwnerAccess")]
    pub r#bucket_owner_access: Option<String>,
    /// Whether to disable encrypting S3 logs. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "encryptionDisabled")]
    pub r#encryption_disabled: Option<bool>,
    /// Name of the S3 bucket and the path prefix for S3 logs. Must be set if status is `ENABLED`, otherwise it must be empty.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// Current status of logs in S3 for a build project. Valid values: `ENABLED`, `DISABLED`. Defaults to `DISABLED`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
