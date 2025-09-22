#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InsightsReportConfigObjectMetadataReportOptions {
    /// The metadata fields included in an inventory report.
    #[builder(into)]
    #[serde(rename = "metadataFields")]
    pub r#metadata_fields: Vec<String>,
    /// Options for where the inventory reports are stored.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "storageDestinationOptions")]
    pub r#storage_destination_options: Box<super::super::types::storage::InsightsReportConfigObjectMetadataReportOptionsStorageDestinationOptions>,
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "storageFilters")]
    pub r#storage_filters: Box<Option<super::super::types::storage::InsightsReportConfigObjectMetadataReportOptionsStorageFilters>>,
}
