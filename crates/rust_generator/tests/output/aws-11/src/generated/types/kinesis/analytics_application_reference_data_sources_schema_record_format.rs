#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AnalyticsApplicationReferenceDataSourcesSchemaRecordFormat {
    /// The Mapping Information for the record format.
    /// See Mapping Parameters below for more details.
    #[builder(into)]
    #[serde(rename = "mappingParameters")]
    pub r#mapping_parameters: Option<Box<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParameters>>,
    /// The type of Record Format. Can be `CSV` or `JSON`.
    #[builder(into)]
    #[serde(rename = "recordFormatType")]
    pub r#record_format_type: Option<String>,
}
