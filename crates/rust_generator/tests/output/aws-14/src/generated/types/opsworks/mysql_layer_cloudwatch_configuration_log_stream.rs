#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MysqlLayerCloudwatchConfigurationLogStream {
    #[builder(into)]
    #[serde(rename = "batchCount")]
    pub r#batch_count: Option<i32>,
    #[builder(into)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Option<i32>,
    #[builder(into)]
    #[serde(rename = "bufferDuration")]
    pub r#buffer_duration: Option<i32>,
    #[builder(into)]
    #[serde(rename = "datetimeFormat")]
    pub r#datetime_format: Option<String>,
    #[builder(into)]
    #[serde(rename = "encoding")]
    pub r#encoding: Option<String>,
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: String,
    #[builder(into)]
    #[serde(rename = "fileFingerprintLines")]
    pub r#file_fingerprint_lines: Option<String>,
    #[builder(into)]
    #[serde(rename = "initialPosition")]
    pub r#initial_position: Option<String>,
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: String,
    #[builder(into)]
    #[serde(rename = "multilineStartPattern")]
    pub r#multiline_start_pattern: Option<String>,
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
}
