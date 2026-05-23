#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobQuery {
    /// If true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance.
    /// Requires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed.
    /// However, you must still set destinationTable when result size exceeds the allowed maximum response size.
    #[builder(into)]
    pub r#allow_large_results: Option<bool>,
    /// Specifies whether the job is allowed to create new tables. The following values are supported:
    /// CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.
    /// CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.
    /// Creation, truncation and append actions occur as one atomic update upon job completion
    /// Default value is `CREATE_IF_NEEDED`.
    /// Possible values are: `CREATE_IF_NEEDED`, `CREATE_NEVER`.
    #[builder(into)]
    pub r#create_disposition: Option<String>,
    /// Specifies the default dataset to use for unqualified table names in the query. Note that this does not alter behavior of unqualified dataset names.
    /// Structure is documented below.
    #[builder(into)]
    pub r#default_dataset: Option<Box<super::super::types::bigquery::JobQueryDefaultDataset>>,
    /// Custom encryption configuration (e.g., Cloud KMS keys)
    /// Structure is documented below.
    #[builder(into)]
    pub r#destination_encryption_configuration: Option<Box<super::super::types::bigquery::JobQueryDestinationEncryptionConfiguration>>,
    /// Describes the table where the query results should be stored.
    /// This property must be set for large results that exceed the maximum response size.
    /// For queries that produce anonymous (cached) results, this field will be populated by BigQuery.
    /// Structure is documented below.
    #[builder(into)]
    pub r#destination_table: Option<Box<super::super::types::bigquery::JobQueryDestinationTable>>,
    /// If true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results.
    /// allowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened.
    #[builder(into)]
    pub r#flatten_results: Option<bool>,
    /// Limits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge).
    /// If unspecified, this will be set to your project default.
    #[builder(into)]
    pub r#maximum_billing_tier: Option<i32>,
    /// Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge).
    /// If unspecified, this will be set to your project default.
    #[builder(into)]
    pub r#maximum_bytes_billed: Option<String>,
    /// Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query.
    #[builder(into)]
    pub r#parameter_mode: Option<String>,
    /// Specifies a priority for the query.
    /// Default value is `INTERACTIVE`.
    /// Possible values are: `INTERACTIVE`, `BATCH`.
    #[builder(into)]
    pub r#priority: Option<String>,
    /// SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL.
    /// *NOTE*: queries containing [DML language](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language)
    /// (`DELETE`, `UPDATE`, `MERGE`, `INSERT`) must specify `create_disposition = ""` and `write_disposition = ""`.
    #[builder(into)]
    pub r#query: String,
    /// Allows the schema of the destination table to be updated as a side effect of the query job.
    /// Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND;
    /// when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table,
    /// specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema.
    /// One or more of the following values are specified:
    /// ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema.
    /// ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    #[builder(into)]
    pub r#schema_update_options: Option<Vec<String>>,
    /// Options controlling the execution of scripts.
    /// Structure is documented below.
    #[builder(into)]
    pub r#script_options: Option<Box<super::super::types::bigquery::JobQueryScriptOptions>>,
    /// Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true.
    /// If set to false, the query will use BigQuery's standard SQL.
    #[builder(into)]
    pub r#use_legacy_sql: Option<bool>,
    /// Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever
    /// tables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified.
    /// The default value is true.
    #[builder(into)]
    pub r#use_query_cache: Option<bool>,
    /// Describes user-defined function resources used in the query.
    /// Structure is documented below.
    #[builder(into)]
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
    pub r#write_disposition: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobQuery {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "allowLargeResults",
                    &self.r#allow_large_results,
                ),
                to_pulumi_object_field(
                    "createDisposition",
                    &self.r#create_disposition,
                ),
                to_pulumi_object_field(
                    "defaultDataset",
                    &self.r#default_dataset,
                ),
                to_pulumi_object_field(
                    "destinationEncryptionConfiguration",
                    &self.r#destination_encryption_configuration,
                ),
                to_pulumi_object_field(
                    "destinationTable",
                    &self.r#destination_table,
                ),
                to_pulumi_object_field(
                    "flattenResults",
                    &self.r#flatten_results,
                ),
                to_pulumi_object_field(
                    "maximumBillingTier",
                    &self.r#maximum_billing_tier,
                ),
                to_pulumi_object_field(
                    "maximumBytesBilled",
                    &self.r#maximum_bytes_billed,
                ),
                to_pulumi_object_field(
                    "parameterMode",
                    &self.r#parameter_mode,
                ),
                to_pulumi_object_field(
                    "priority",
                    &self.r#priority,
                ),
                to_pulumi_object_field(
                    "query",
                    &self.r#query,
                ),
                to_pulumi_object_field(
                    "schemaUpdateOptions",
                    &self.r#schema_update_options,
                ),
                to_pulumi_object_field(
                    "scriptOptions",
                    &self.r#script_options,
                ),
                to_pulumi_object_field(
                    "useLegacySql",
                    &self.r#use_legacy_sql,
                ),
                to_pulumi_object_field(
                    "useQueryCache",
                    &self.r#use_query_cache,
                ),
                to_pulumi_object_field(
                    "userDefinedFunctionResources",
                    &self.r#user_defined_function_resources,
                ),
                to_pulumi_object_field(
                    "writeDisposition",
                    &self.r#write_disposition,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobQuery {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#allow_large_results: {
                        let field_value = match fields_map.get("allowLargeResults") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowLargeResults' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_disposition: {
                        let field_value = match fields_map.get("createDisposition") {
                            Some(value) => value,
                            None => bail!("Missing field 'createDisposition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_dataset: {
                        let field_value = match fields_map.get("defaultDataset") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultDataset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_encryption_configuration: {
                        let field_value = match fields_map.get("destinationEncryptionConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationEncryptionConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_table: {
                        let field_value = match fields_map.get("destinationTable") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationTable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flatten_results: {
                        let field_value = match fields_map.get("flattenResults") {
                            Some(value) => value,
                            None => bail!("Missing field 'flattenResults' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_billing_tier: {
                        let field_value = match fields_map.get("maximumBillingTier") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximumBillingTier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_bytes_billed: {
                        let field_value = match fields_map.get("maximumBytesBilled") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximumBytesBilled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameter_mode: {
                        let field_value = match fields_map.get("parameterMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameterMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query: {
                        let field_value = match fields_map.get("query") {
                            Some(value) => value,
                            None => bail!("Missing field 'query' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_update_options: {
                        let field_value = match fields_map.get("schemaUpdateOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'schemaUpdateOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#script_options: {
                        let field_value = match fields_map.get("scriptOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'scriptOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_legacy_sql: {
                        let field_value = match fields_map.get("useLegacySql") {
                            Some(value) => value,
                            None => bail!("Missing field 'useLegacySql' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_query_cache: {
                        let field_value = match fields_map.get("useQueryCache") {
                            Some(value) => value,
                            None => bail!("Missing field 'useQueryCache' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_defined_function_resources: {
                        let field_value = match fields_map.get("userDefinedFunctionResources") {
                            Some(value) => value,
                            None => bail!("Missing field 'userDefinedFunctionResources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_disposition: {
                        let field_value = match fields_map.get("writeDisposition") {
                            Some(value) => value,
                            None => bail!("Missing field 'writeDisposition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
