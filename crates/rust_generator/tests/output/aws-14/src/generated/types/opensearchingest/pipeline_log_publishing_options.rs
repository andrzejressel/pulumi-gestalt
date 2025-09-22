#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipelineLogPublishingOptions {
    /// The destination for OpenSearch Ingestion logs sent to Amazon CloudWatch Logs. This parameter is required if IsLoggingEnabled is set to true. See `cloudwatch_log_destination` below.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogDestination")]
    pub r#cloudwatch_log_destination: Box<Option<super::super::types::opensearchingest::PipelineLogPublishingOptionsCloudwatchLogDestination>>,
    /// Whether logs should be published.
    #[builder(into)]
    #[serde(rename = "isLoggingEnabled")]
    pub r#is_logging_enabled: Option<bool>,
}
