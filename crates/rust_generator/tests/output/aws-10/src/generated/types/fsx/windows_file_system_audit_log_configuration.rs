#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsFileSystemAuditLogConfiguration {
    /// The Amazon Resource Name (ARN) for the destination of the audit logs. The destination can be any Amazon CloudWatch Logs log group ARN or Amazon Kinesis Data Firehose delivery stream ARN. Can be specified when `file_access_audit_log_level` and `file_share_access_audit_log_level` are not set to `DISABLED`. The name of the Amazon CloudWatch Logs log group must begin with the `/aws/fsx` prefix. The name of the Amazon Kinesis Data Firehouse delivery stream must begin with the `aws-fsx` prefix. If you do not provide a destination in `audit_log_destionation`, Amazon FSx will create and use a log stream in the CloudWatch Logs /aws/fsx/windows log group.
    #[builder(into)]
    #[serde(rename = "auditLogDestination")]
    pub r#audit_log_destination: Option<String>,
    /// Sets which attempt type is logged by Amazon FSx for file and folder accesses. Valid values are `SUCCESS_ONLY`, `FAILURE_ONLY`, `SUCCESS_AND_FAILURE`, and `DISABLED`. Default value is `DISABLED`.
    #[builder(into)]
    #[serde(rename = "fileAccessAuditLogLevel")]
    pub r#file_access_audit_log_level: Option<String>,
    /// Sets which attempt type is logged by Amazon FSx for file share accesses. Valid values are `SUCCESS_ONLY`, `FAILURE_ONLY`, `SUCCESS_AND_FAILURE`, and `DISABLED`. Default value is `DISABLED`.
    #[builder(into)]
    #[serde(rename = "fileShareAccessAuditLogLevel")]
    pub r#file_share_access_audit_log_level: Option<String>,
}
