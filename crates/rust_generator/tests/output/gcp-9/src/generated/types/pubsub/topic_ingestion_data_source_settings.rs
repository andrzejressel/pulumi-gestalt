#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicIngestionDataSourceSettings {
    /// Settings for ingestion from Amazon Kinesis Data Streams.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "awsKinesis")]
    pub r#aws_kinesis: Option<Box<super::super::types::pubsub::TopicIngestionDataSourceSettingsAwsKinesis>>,
    /// Settings for ingestion from Cloud Storage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudStorage")]
    pub r#cloud_storage: Option<Box<super::super::types::pubsub::TopicIngestionDataSourceSettingsCloudStorage>>,
    /// Settings for Platform Logs regarding ingestion to Pub/Sub. If unset,
    /// no Platform Logs will be generated.'
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "platformLogsSettings")]
    pub r#platform_logs_settings: Option<Box<super::super::types::pubsub::TopicIngestionDataSourceSettingsPlatformLogsSettings>>,
}
