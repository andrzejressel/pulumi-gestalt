#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamSourceConfigOracleSourceConfigExcludeObjectsOracleSchema {
    /// Tables in the database.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oracleTables")]
    pub r#oracle_tables: Option<Vec<super::super::types::datastream::StreamSourceConfigOracleSourceConfigExcludeObjectsOracleSchemaOracleTable>>,
    /// Schema name.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: String,
}
