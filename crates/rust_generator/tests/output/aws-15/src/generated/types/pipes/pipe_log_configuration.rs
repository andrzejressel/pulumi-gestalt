#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeLogConfiguration {
    /// Amazon CloudWatch Logs logging configuration settings for the pipe. Detailed below.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogsLogDestination")]
    pub r#cloudwatch_logs_log_destination: Option<Box<super::super::types::pipes::PipeLogConfigurationCloudwatchLogsLogDestination>>,
    /// Amazon Kinesis Data Firehose logging configuration settings for the pipe. Detailed below.
    #[builder(into)]
    #[serde(rename = "firehoseLogDestination")]
    pub r#firehose_log_destination: Option<Box<super::super::types::pipes::PipeLogConfigurationFirehoseLogDestination>>,
    /// String list that specifies whether the execution data (specifically, the `payload`, `awsRequest`, and `awsResponse` fields) is included in the log messages for this pipe. This applies to all log destinations for the pipe. Valid values `ALL`.
    #[builder(into)]
    #[serde(rename = "includeExecutionDatas")]
    pub r#include_execution_datas: Option<Vec<String>>,
    /// The level of logging detail to include. Valid values `OFF`, `ERROR`, `INFO` and `TRACE`.
    #[builder(into)]
    #[serde(rename = "level")]
    pub r#level: String,
    /// Amazon S3 logging configuration settings for the pipe. Detailed below.
    #[builder(into)]
    #[serde(rename = "s3LogDestination")]
    pub r#s_3_log_destination: Option<Box<super::super::types::pipes::PipeLogConfigurationS3LogDestination>>,
}
