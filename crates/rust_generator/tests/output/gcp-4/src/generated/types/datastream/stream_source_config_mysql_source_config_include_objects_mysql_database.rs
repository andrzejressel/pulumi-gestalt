#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamSourceConfigMysqlSourceConfigIncludeObjectsMysqlDatabase {
    /// Database name.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: String,
    /// Tables in the database.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mysqlTables")]
    pub r#mysql_tables: Option<Vec<super::super::types::datastream::StreamSourceConfigMysqlSourceConfigIncludeObjectsMysqlDatabaseMysqlTable>>,
}
