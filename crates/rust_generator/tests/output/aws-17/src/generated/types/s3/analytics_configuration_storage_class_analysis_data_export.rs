#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnalyticsConfigurationStorageClassAnalysisDataExport {
    /// Specifies the destination for the exported analytics data (documented below).
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::s3::AnalyticsConfigurationStorageClassAnalysisDataExportDestination>,
    /// Schema version of exported analytics data. Allowed values: `V_1`. Default value: `V_1`.
    #[builder(into)]
    #[serde(rename = "outputSchemaVersion")]
    pub r#output_schema_version: Option<String>,
}
