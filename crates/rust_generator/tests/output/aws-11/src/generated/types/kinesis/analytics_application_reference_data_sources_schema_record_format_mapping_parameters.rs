#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParameters {
    /// Mapping information when the record format uses delimiters.
    /// See CSV Mapping Parameters below for more details.
    #[builder(into)]
    #[serde(rename = "csv")]
    pub r#csv: Option<Box<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParametersCsv>>,
    /// Mapping information when JSON is the record format on the streaming source.
    /// See JSON Mapping Parameters below for more details.
    #[builder(into)]
    #[serde(rename = "json")]
    pub r#json: Option<Box<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParametersJson>>,
}
