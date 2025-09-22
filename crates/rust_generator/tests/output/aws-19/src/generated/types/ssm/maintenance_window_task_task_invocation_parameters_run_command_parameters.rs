#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters {
    /// Configuration options for sending command output to CloudWatch Logs. Documented below.
    #[builder(into)]
    #[serde(rename = "cloudwatchConfig")]
    pub r#cloudwatch_config: Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersCloudwatchConfig>>,
    /// Information about the command(s) to execute.
    #[builder(into)]
    #[serde(rename = "comment")]
    pub r#comment: Option<String>,
    /// The SHA-256 or SHA-1 hash created by the system when the document was created. SHA-1 hashes have been deprecated.
    #[builder(into)]
    #[serde(rename = "documentHash")]
    pub r#document_hash: Option<String>,
    /// SHA-256 or SHA-1. SHA-1 hashes have been deprecated. Valid values: `Sha256` and `Sha1`
    #[builder(into)]
    #[serde(rename = "documentHashType")]
    pub r#document_hash_type: Option<String>,
    /// The version of an Automation document to use during task execution.
    #[builder(into)]
    #[serde(rename = "documentVersion")]
    pub r#document_version: Option<String>,
    /// Configurations for sending notifications about command status changes on a per-instance basis. Documented below.
    #[builder(into)]
    #[serde(rename = "notificationConfig")]
    pub r#notification_config: Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfig>>,
    /// The name of the Amazon S3 bucket.
    #[builder(into)]
    #[serde(rename = "outputS3Bucket")]
    pub r#output_s_3_bucket: Option<String>,
    /// The Amazon S3 bucket subfolder.
    #[builder(into)]
    #[serde(rename = "outputS3KeyPrefix")]
    pub r#output_s_3_key_prefix: Option<String>,
    /// The parameters for the RUN_COMMAND task execution. Documented below.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameter>>,
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) service role to use to publish Amazon Simple Notification Service (Amazon SNS) notifications for maintenance window Run Command tasks.
    #[builder(into)]
    #[serde(rename = "serviceRoleArn")]
    pub r#service_role_arn: Option<String>,
    /// If this time is reached and the command has not already started executing, it doesn't run.
    #[builder(into)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Option<i32>,
}
