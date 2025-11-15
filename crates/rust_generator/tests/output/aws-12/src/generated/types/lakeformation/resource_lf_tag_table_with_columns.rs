#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourceLfTagTableWithColumns {
    /// Identifier for the Data Catalog. By default, it is the account ID of the caller.
    #[builder(into)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Option<String>,
    /// Set of column names for the table.
    #[builder(into)]
    #[serde(rename = "columnNames")]
    pub r#column_names: Option<Vec<String>>,
    /// Option to add column wildcard. See Column Wildcard for more details.
    #[builder(into)]
    #[serde(rename = "columnWildcard")]
    pub r#column_wildcard: Option<Box<super::super::types::lakeformation::ResourceLfTagTableWithColumnsColumnWildcard>>,
    /// Name of the database for the table with columns resource. Unique to the Data Catalog.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// Name of the table resource.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
