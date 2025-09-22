#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataQualityRulesetTargetTable {
    /// The catalog id where the AWS Glue table exists.
    #[builder(into)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Option<String>,
    /// Name of the database where the AWS Glue table exists.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// Name of the AWS Glue table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: String,
}
