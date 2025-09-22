#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoggingConfigurationDestinationConfiguration {
    /// An Amazon CloudWatch Logs destination configuration where chat activity will be logged.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Option<Box<super::super::types::ivschat::LoggingConfigurationDestinationConfigurationCloudwatchLogs>>,
    /// An Amazon Kinesis Data Firehose destination configuration where chat activity will be logged.
    #[builder(into)]
    #[serde(rename = "firehose")]
    pub r#firehose: Option<Box<super::super::types::ivschat::LoggingConfigurationDestinationConfigurationFirehose>>,
    /// An Amazon S3 destination configuration where chat activity will be logged.
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Option<Box<super::super::types::ivschat::LoggingConfigurationDestinationConfigurationS3>>,
}
