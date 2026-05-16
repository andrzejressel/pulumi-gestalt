#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfigurationCloudWatchMonitoringConfiguration {
    /// The name of the log group for log publishing.
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: String,
    /// The specified name prefix for log streams.
    #[builder(into)]
    #[serde(rename = "logStreamNamePrefix")]
    pub r#log_stream_name_prefix: Option<String>,
}
