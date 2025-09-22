#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobQuery {
    /// If true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance.
    /// Requires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed.
    /// However, you must still set destinationTable when result size exceeds the allowed maximum response size.
    #[builder(into)]
    #[serde(rename = "allowLargeResults")]
    pub r#allow_large_results: Option<bool>,
    /// Specifies whether the job is allowed to create new tables. The following values are supported:
    /// CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.
    /// CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.
    /// Creation, truncation and append actions occur as one atomic update upon job completion
    /// Default value is `CREATE_IF_NEEDED`.
    /// Possible values are: `CREATE_IF_NEEDED`, `CREATE_NEVER`.
    #[builder(into)]
    #[serde(rename = "createDisposition")]
    pub r#create_disposition: Option<String>,
    /// Specifies the default dataset to use for unqualified table names in the query. Note that this does not alter behavior of unqualified dataset names.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "defaultDataset")]
    pub r#default_dataset: Box<Option<super::super::types::bigquery::JobQueryDefaultDataset>>,
    /// Custom encryption configuration (e.g., Cloud KMS keys)
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destinationEncryptionConfiguration")]
    pub r#destination_encryption_configuration: Box<Option<super::super::types::bigquery::JobQueryDestinationEncryptionConfiguration>>,
    /// Describes the table where the query results should be stored.
    /// This property must be set for large results that exceed the maximum response size.
    /// For queries that produce anonymous (cached) results, this field will be populated by BigQuery.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destinationTable")]
    pub r#destination_table: Box<Option<super::super::types::bigquery::JobQueryDestinationTable>>,
    /// If true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results.
    /// allowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened.
    #[builder(into)]
    #[serde(rename = "flattenResults")]
    pub r#flatten_results: Option<bool>,
    /// Limits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge).
    /// If unspecified, this will be set to your project default.
    #[builder(into)]
    #[serde(rename = "maximumBillingTier")]
    pub r#maximum_billing_tier: Option<i32>,
    /// Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge).
    /// If unspecified, this will be set to your project default.
    #[builder(into)]
    #[serde(rename = "maximumBytesBilled")]
    pub r#maximum_bytes_billed: Option<String>,
    /// Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query.
    #[builder(into)]
    #[serde(rename = "parameterMode")]
    pub r#parameter_mode: Option<String>,
    /// Specifies a priority for the query.
    /// Default value is `INTERACTIVE`.
    /// Possible values are: `INTERACTIVE`, `BATCH`.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<String>,
    /// SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL.
    /// *NOTE*: queries containing [DML language](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language)
    /// (`DELETE`, `UPDATE`, `MERGE`, `INSERT`) must specify `create_disposition = ""` and `write_disposition = ""`.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: String,
    /// Allows the schema of the destination table to be updated as a side effect of the query job.
    /// Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND;
    /// when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table,
    /// specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema.
    /// One or more of the following values are specified:
    /// ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema.
    /// ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    #[builder(into)]
    #[serde(rename = "schemaUpdateOptions")]
    pub r#schema_update_options: Option<Vec<String>>,
    /// Options controlling the execution of scripts.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "scriptOptions")]
    pub r#script_options: Box<Option<super::super::types::bigquery::JobQueryScriptOptions>>,
    /// Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true.
    /// If set to false, the query will use BigQuery's standard SQL.
    #[builder(into)]
    #[serde(rename = "useLegacySql")]
    pub r#use_legacy_sql: Option<bool>,
    /// Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever
    /// tables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified.
    /// The default value is true.
    #[builder(into)]
    #[serde(rename = "useQueryCache")]
    pub r#use_query_cache: Option<bool>,
    /// Describes user-defined function resources used in the query.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "userDefinedFunctionResources")]
    pub r#user_defined_function_resources: Option<Vec<super::super::types::bigquery::JobQueryUserDefinedFunctionResource>>,
    /// Specifies the action that occurs if the destination table already exists. The following values are supported:
    /// WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.
    /// WRITE_APPEND: If the table already exists, BigQuery appends the data to the table.
    /// WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.
    /// Each action is atomic and only occurs if BigQuery is able to complete the job successfully.
    /// Creation, truncation and append actions occur as one atomic update upon job completion.
    /// Default value is `WRITE_EMPTY`.
    /// Possible values are: `WRITE_TRUNCATE`, `WRITE_APPEND`, `WRITE_EMPTY`.
    #[builder(into)]
    #[serde(rename = "writeDisposition")]
    pub r#write_disposition: Option<String>,
}
