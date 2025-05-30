#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamBackfillAllPostgresqlExcludedObjectsPostgresqlSchema {
    /// Tables in the schema.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "postgresqlTables")]
    pub r#postgresql_tables: Box<Option<Vec<super::super::types::datastream::StreamBackfillAllPostgresqlExcludedObjectsPostgresqlSchemaPostgresqlTable>>>,
    /// Database name.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Box<String>,
}
