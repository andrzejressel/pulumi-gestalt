#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchemaRecordFormatMappingParametersCsvMappingParameters {
    /// The column delimiter. For example, in a CSV format, a comma (`,`) is the typical column delimiter.
    #[builder(into)]
    #[serde(rename = "recordColumnDelimiter")]
    pub r#record_column_delimiter: String,
    /// The row delimiter. For example, in a CSV format, `\n` is the typical row delimiter.
    #[builder(into)]
    #[serde(rename = "recordRowDelimiter")]
    pub r#record_row_delimiter: String,
}
