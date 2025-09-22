#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParametersJson {
    /// Path to the top-level parent that contains the records.
    #[builder(into)]
    #[serde(rename = "recordRowPath")]
    pub r#record_row_path: String,
}
