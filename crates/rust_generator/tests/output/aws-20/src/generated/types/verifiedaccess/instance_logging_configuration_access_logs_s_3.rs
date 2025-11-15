#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceLoggingConfigurationAccessLogsS3 {
    /// The name of S3 bucket.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Option<String>,
    /// The ID of the AWS account that owns the Amazon S3 bucket.
    #[builder(into)]
    #[serde(rename = "bucketOwner")]
    pub r#bucket_owner: Option<String>,
    /// Indicates whether logging is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// The bucket prefix.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
}
