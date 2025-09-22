#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ProjectLogsConfig {
    /// Configuration block. Detailed below.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Option<Box<super::super::types::codebuild::ProjectLogsConfigCloudwatchLogs>>,
    /// Configuration block. Detailed below.
    #[builder(into)]
    #[serde(rename = "s3Logs")]
    pub r#s_3_logs: Option<Box<super::super::types::codebuild::ProjectLogsConfigS3Logs>>,
}
