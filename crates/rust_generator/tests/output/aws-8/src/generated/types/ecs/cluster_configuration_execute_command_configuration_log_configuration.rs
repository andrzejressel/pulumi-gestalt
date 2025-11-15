#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterConfigurationExecuteCommandConfigurationLogConfiguration {
    /// Whether to enable encryption on the CloudWatch logs. If not specified, encryption will be disabled.
    #[builder(into)]
    #[serde(rename = "cloudWatchEncryptionEnabled")]
    pub r#cloud_watch_encryption_enabled: Option<bool>,
    /// The name of the CloudWatch log group to send logs to.
    #[builder(into)]
    #[serde(rename = "cloudWatchLogGroupName")]
    pub r#cloud_watch_log_group_name: Option<String>,
    /// Whether to enable encryption on the logs sent to S3. If not specified, encryption will be disabled.
    #[builder(into)]
    #[serde(rename = "s3BucketEncryptionEnabled")]
    pub r#s_3_bucket_encryption_enabled: Option<bool>,
    /// Name of the S3 bucket to send logs to.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: Option<String>,
    /// Optional folder in the S3 bucket to place logs in.
    #[builder(into)]
    #[serde(rename = "s3KeyPrefix")]
    pub r#s_3_key_prefix: Option<String>,
}
