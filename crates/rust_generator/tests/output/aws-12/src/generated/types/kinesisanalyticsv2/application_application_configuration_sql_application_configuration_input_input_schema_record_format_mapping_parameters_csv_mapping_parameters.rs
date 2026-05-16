#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
