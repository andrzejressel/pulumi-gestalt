#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProjectLogsConfigCloudwatchLogs {
    /// Group name of the logs in CloudWatch Logs.
    #[builder(into)]
    #[serde(rename = "groupName")]
    pub r#group_name: Option<String>,
    /// Current status of logs in CloudWatch Logs for a build project. Valid values: `ENABLED`, `DISABLED`. Defaults to `ENABLED`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// Prefix of the log stream name of the logs in CloudWatch Logs.
    #[builder(into)]
    #[serde(rename = "streamName")]
    pub r#stream_name: Option<String>,
}
