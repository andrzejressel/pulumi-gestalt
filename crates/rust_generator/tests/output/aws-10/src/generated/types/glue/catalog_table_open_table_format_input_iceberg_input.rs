#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CatalogTableOpenTableFormatInputIcebergInput {
    /// A required metadata operation. Can only be set to CREATE.
    #[builder(into)]
    #[serde(rename = "metadataOperation")]
    pub r#metadata_operation: String,
    /// The table version for the Iceberg table. Defaults to 2.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
