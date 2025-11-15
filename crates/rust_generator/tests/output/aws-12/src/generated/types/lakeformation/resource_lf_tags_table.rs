#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourceLfTagsTable {
    /// Identifier for the Data Catalog. By default, it is the account ID of the caller.
    #[builder(into)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Option<String>,
    /// Name of the database for the table. Unique to a Data Catalog.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// Name of the table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Whether to use a wildcard representing every table under a database. Defaults to `false`.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "wildcard")]
    pub r#wildcard: Option<bool>,
}
