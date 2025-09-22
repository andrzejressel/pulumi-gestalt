#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetMetastoreServiceMetadataIntegration {
    /// The integration config for the Data Catalog service.
    #[builder(into)]
    #[serde(rename = "dataCatalogConfigs")]
    pub r#data_catalog_configs: Vec<super::super::types::dataproc::GetMetastoreServiceMetadataIntegrationDataCatalogConfig>,
}
