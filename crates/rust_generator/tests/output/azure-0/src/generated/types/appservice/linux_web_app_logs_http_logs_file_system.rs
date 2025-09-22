#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinuxWebAppLogsHttpLogsFileSystem {
    /// The retention period in days. A value of `0` means no retention.
    #[builder(into)]
    #[serde(rename = "retentionInDays")]
    pub r#retention_in_days: i32,
    /// The maximum size in megabytes that log files can use.
    #[builder(into)]
    #[serde(rename = "retentionInMb")]
    pub r#retention_in_mb: i32,
}
