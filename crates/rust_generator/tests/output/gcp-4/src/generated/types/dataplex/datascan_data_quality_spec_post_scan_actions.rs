#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatascanDataQualitySpecPostScanActions {
    /// If set, results will be exported to the provided BigQuery table.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bigqueryExport")]
    pub r#bigquery_export: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecPostScanActionsBigqueryExport>>,
}
