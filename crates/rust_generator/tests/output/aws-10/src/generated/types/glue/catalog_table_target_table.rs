#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CatalogTableTargetTable {
    /// ID of the Data Catalog in which the table resides.
    #[builder(into)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: String,
    /// Name of the catalog database that contains the target table.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// Name of the target table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Region of the target table.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
}
