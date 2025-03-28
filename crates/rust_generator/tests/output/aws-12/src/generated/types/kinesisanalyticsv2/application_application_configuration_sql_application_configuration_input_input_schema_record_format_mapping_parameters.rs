#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchemaRecordFormatMappingParameters {
    /// Provides additional mapping information when the record format uses delimiters (for example, CSV).
    #[builder(into, default)]
    #[serde(rename = "csvMappingParameters")]
    pub r#csv_mapping_parameters: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchemaRecordFormatMappingParametersCsvMappingParameters>>,
    /// Provides additional mapping information when JSON is the record format on the streaming source.
    #[builder(into, default)]
    #[serde(rename = "jsonMappingParameters")]
    pub r#json_mapping_parameters: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchemaRecordFormatMappingParametersJsonMappingParameters>>,
}
