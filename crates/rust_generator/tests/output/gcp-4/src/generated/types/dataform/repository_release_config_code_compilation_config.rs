#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryReleaseConfigCodeCompilationConfig {
    /// Optional. The default schema (BigQuery dataset ID) for assertions.
    #[builder(into)]
    #[serde(rename = "assertionSchema")]
    pub r#assertion_schema: Option<String>,
    /// Optional. The suffix that should be appended to all database (Google Cloud project ID) names.
    #[builder(into)]
    #[serde(rename = "databaseSuffix")]
    pub r#database_suffix: Option<String>,
    /// Optional. The default database (Google Cloud project ID).
    #[builder(into)]
    #[serde(rename = "defaultDatabase")]
    pub r#default_database: Option<String>,
    /// Optional. The default BigQuery location to use. Defaults to "US".
    /// See the BigQuery docs for a full list of locations: https://cloud.google.com/bigquery/docs/locations.
    #[builder(into)]
    #[serde(rename = "defaultLocation")]
    pub r#default_location: Option<String>,
    /// Optional. The default schema (BigQuery dataset ID).
    #[builder(into)]
    #[serde(rename = "defaultSchema")]
    pub r#default_schema: Option<String>,
    /// Optional. The suffix that should be appended to all schema (BigQuery dataset ID) names.
    #[builder(into)]
    #[serde(rename = "schemaSuffix")]
    pub r#schema_suffix: Option<String>,
    /// Optional. The prefix that should be prepended to all table names.
    #[builder(into)]
    #[serde(rename = "tablePrefix")]
    pub r#table_prefix: Option<String>,
    /// Optional. User-defined variables that are made available to project code during compilation.
    /// An object containing a list of "key": value pairs.
    /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[builder(into)]
    #[serde(rename = "vars")]
    pub r#vars: Option<std::collections::HashMap<String, String>>,
}
