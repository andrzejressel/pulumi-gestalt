#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLogDataProtectionPolicyDocumentStatementOperationAuditFindingsDestination {
    /// Configures CloudWatch Logs as a findings destination.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Option<Box<super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatementOperationAuditFindingsDestinationCloudwatchLogs>>,
    /// Configures Kinesis Firehose as a findings destination.
    #[builder(into)]
    #[serde(rename = "firehose")]
    pub r#firehose: Option<Box<super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatementOperationAuditFindingsDestinationFirehose>>,
    /// Configures S3 as a findings destination.
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Option<Box<super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatementOperationAuditFindingsDestinationS3>>,
}
