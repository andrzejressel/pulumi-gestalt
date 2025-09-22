#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamBackfillAllSqlServerExcludedObjectsSchema {
    /// Schema name.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: String,
    /// Tables in the database.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tables")]
    pub r#tables: Option<Vec<super::super::types::datastream::StreamBackfillAllSqlServerExcludedObjectsSchemaTable>>,
}
