#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParametersCsv {
    /// The Column Delimiter.
    #[builder(into)]
    #[serde(rename = "recordColumnDelimiter")]
    pub r#record_column_delimiter: String,
    /// The Row Delimiter.
    #[builder(into)]
    #[serde(rename = "recordRowDelimiter")]
    pub r#record_row_delimiter: String,
}
