#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTopicIngestionDataSourceSetting {
    /// Settings for ingestion from Amazon Kinesis Data Streams.
    #[builder(into)]
    #[serde(rename = "awsKineses")]
    pub r#aws_kineses: Vec<super::super::types::pubsub::GetTopicIngestionDataSourceSettingAwsKinese>,
    /// Settings for ingestion from Cloud Storage.
    #[builder(into)]
    #[serde(rename = "cloudStorages")]
    pub r#cloud_storages: Vec<super::super::types::pubsub::GetTopicIngestionDataSourceSettingCloudStorage>,
    /// Settings for Platform Logs regarding ingestion to Pub/Sub. If unset,
    /// no Platform Logs will be generated.'
    #[builder(into)]
    #[serde(rename = "platformLogsSettings")]
    pub r#platform_logs_settings: Vec<super::super::types::pubsub::GetTopicIngestionDataSourceSettingPlatformLogsSetting>,
}
