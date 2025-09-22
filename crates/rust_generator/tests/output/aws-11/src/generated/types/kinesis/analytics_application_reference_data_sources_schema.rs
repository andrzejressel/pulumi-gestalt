#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AnalyticsApplicationReferenceDataSourcesSchema {
    /// The Record Column mapping for the streaming source data element.
    /// See Record Columns below for more details.
    #[builder(into)]
    #[serde(rename = "recordColumns")]
    pub r#record_columns: Vec<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesSchemaRecordColumn>,
    /// The Encoding of the record in the streaming source.
    #[builder(into)]
    #[serde(rename = "recordEncoding")]
    pub r#record_encoding: Option<String>,
    /// The Record Format and mapping information to schematize a record.
    /// See Record Format below for more details.
    #[builder(into)]
    #[serde(rename = "recordFormat")]
    pub r#record_format: Box<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesSchemaRecordFormat>,
}
