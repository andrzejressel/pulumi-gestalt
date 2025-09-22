#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirehoseDeliveryStreamOpensearchConfigurationS3ConfigurationCloudwatchLoggingOptions {
    /// Enables or disables the logging. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The CloudWatch group name for logging. This value is required if `enabled` is true.
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: Option<String>,
    /// The CloudWatch log stream name for logging. This value is required if `enabled` is true.
    #[builder(into)]
    #[serde(rename = "logStreamName")]
    pub r#log_stream_name: Option<String>,
}
