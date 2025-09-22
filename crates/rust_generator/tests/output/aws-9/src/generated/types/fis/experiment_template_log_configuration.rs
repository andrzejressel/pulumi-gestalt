#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExperimentTemplateLogConfiguration {
    /// The configuration for experiment logging to Amazon CloudWatch Logs. See below.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogsConfiguration")]
    pub r#cloudwatch_logs_configuration: Option<Box<super::super::types::fis::ExperimentTemplateLogConfigurationCloudwatchLogsConfiguration>>,
    /// The schema version. See [documentation](https://docs.aws.amazon.com/fis/latest/userguide/monitoring-logging.html#experiment-log-schema) for the list of schema versions.
    #[builder(into)]
    #[serde(rename = "logSchemaVersion")]
    pub r#log_schema_version: i32,
    /// The configuration for experiment logging to Amazon S3. See below.
    #[builder(into)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Option<Box<super::super::types::fis::ExperimentTemplateLogConfigurationS3Configuration>>,
}
