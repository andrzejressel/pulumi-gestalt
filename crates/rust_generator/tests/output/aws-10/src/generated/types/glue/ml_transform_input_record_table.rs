#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MlTransformInputRecordTable {
    /// A unique identifier for the AWS Glue Data Catalog.
    #[builder(into)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Option<String>,
    /// The name of the connection to the AWS Glue Data Catalog.
    #[builder(into)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: Option<String>,
    /// A database name in the AWS Glue Data Catalog.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// A table name in the AWS Glue Data Catalog.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: String,
}
