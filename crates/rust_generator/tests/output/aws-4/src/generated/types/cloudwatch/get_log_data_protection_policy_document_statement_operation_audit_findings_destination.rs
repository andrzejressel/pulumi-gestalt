#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLogDataProtectionPolicyDocumentStatementOperationAuditFindingsDestination {
    /// Configures CloudWatch Logs as a findings destination.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Box<Option<super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatementOperationAuditFindingsDestinationCloudwatchLogs>>,
    /// Configures Kinesis Firehose as a findings destination.
    #[builder(into, default)]
    #[serde(rename = "firehose")]
    pub r#firehose: Box<Option<super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatementOperationAuditFindingsDestinationFirehose>>,
    /// Configures S3 as a findings destination.
    #[builder(into, default)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<Option<super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatementOperationAuditFindingsDestinationS3>>,
}
