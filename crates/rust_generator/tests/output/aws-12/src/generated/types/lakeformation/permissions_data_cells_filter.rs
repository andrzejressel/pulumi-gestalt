#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PermissionsDataCellsFilter {
    /// The name of the database.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// The name of the data cells filter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The ID of the Data Catalog.
    #[builder(into)]
    #[serde(rename = "tableCatalogId")]
    pub r#table_catalog_id: String,
    /// The name of the table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: String,
}
