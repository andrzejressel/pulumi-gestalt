#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamBackfillAll {
    /// MySQL data source objects to avoid backfilling.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mysqlExcludedObjects")]
    pub r#mysql_excluded_objects: Option<Box<super::super::types::datastream::StreamBackfillAllMysqlExcludedObjects>>,
    /// PostgreSQL data source objects to avoid backfilling.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oracleExcludedObjects")]
    pub r#oracle_excluded_objects: Option<Box<super::super::types::datastream::StreamBackfillAllOracleExcludedObjects>>,
    /// PostgreSQL data source objects to avoid backfilling.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "postgresqlExcludedObjects")]
    pub r#postgresql_excluded_objects: Option<Box<super::super::types::datastream::StreamBackfillAllPostgresqlExcludedObjects>>,
    /// SQL Server data source objects to avoid backfilling.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sqlServerExcludedObjects")]
    pub r#sql_server_excluded_objects: Option<Box<super::super::types::datastream::StreamBackfillAllSqlServerExcludedObjects>>,
}
