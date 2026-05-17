#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
    pub r#default_dataset: Option<Box<super::super::types::bigquery::JobQueryDefaultDataset>>,
    /// Custom encryption configuration (e.g., Cloud KMS keys)
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destinationEncryptionConfiguration")]
    pub r#destination_encryption_configuration: Option<Box<super::super::types::bigquery::JobQueryDestinationEncryptionConfiguration>>,
    /// Describes the table where the query results should be stored.
    /// This property must be set for large results that exceed the maximum response size.
    /// For queries that produce anonymous (cached) results, this field will be populated by BigQuery.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destinationTable")]
    pub r#destination_table: Option<Box<super::super::types::bigquery::JobQueryDestinationTable>>,
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
    pub r#script_options: Option<Box<super::super::types::bigquery::JobQueryScriptOptions>>,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobQuery {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "allow_large_results".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_large_results,
                )
                .await,
            );
            map.insert(
                "create_disposition".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#create_disposition,
                )
                .await,
            );
            map.insert(
                "default_dataset".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_dataset,
                )
                .await,
            );
            map.insert(
                "destination_encryption_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_encryption_configuration,
                )
                .await,
            );
            map.insert(
                "destination_table".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_table,
                )
                .await,
            );
            map.insert(
                "flatten_results".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#flatten_results,
                )
                .await,
            );
            map.insert(
                "maximum_billing_tier".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_billing_tier,
                )
                .await,
            );
            map.insert(
                "maximum_bytes_billed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_bytes_billed,
                )
                .await,
            );
            map.insert(
                "parameter_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parameter_mode,
                )
                .await,
            );
            map.insert(
                "priority".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#priority,
                )
                .await,
            );
            map.insert(
                "query".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query,
                )
                .await,
            );
            map.insert(
                "schema_update_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_update_options,
                )
                .await,
            );
            map.insert(
                "script_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#script_options,
                )
                .await,
            );
            map.insert(
                "use_legacy_sql".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_legacy_sql,
                )
                .await,
            );
            map.insert(
                "use_query_cache".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_query_cache,
                )
                .await,
            );
            map.insert(
                "user_defined_function_resources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_defined_function_resources,
                )
                .await,
            );
            map.insert(
                "write_disposition".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#write_disposition,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
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
                        let field_value = match fields_map.get("allow_large_results") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_large_results' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_disposition: {
                        let field_value = match fields_map.get("create_disposition") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_disposition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_dataset: {
                        let field_value = match fields_map.get("default_dataset") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_dataset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_encryption_configuration: {
                        let field_value = match fields_map.get("destination_encryption_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_encryption_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_table: {
                        let field_value = match fields_map.get("destination_table") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_table' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flatten_results: {
                        let field_value = match fields_map.get("flatten_results") {
                            Some(value) => value,
                            None => bail!("Missing field 'flatten_results' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_billing_tier: {
                        let field_value = match fields_map.get("maximum_billing_tier") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_billing_tier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_bytes_billed: {
                        let field_value = match fields_map.get("maximum_bytes_billed") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_bytes_billed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameter_mode: {
                        let field_value = match fields_map.get("parameter_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameter_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("schema_update_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_update_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#script_options: {
                        let field_value = match fields_map.get("script_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'script_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_legacy_sql: {
                        let field_value = match fields_map.get("use_legacy_sql") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_legacy_sql' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_query_cache: {
                        let field_value = match fields_map.get("use_query_cache") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_query_cache' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_defined_function_resources: {
                        let field_value = match fields_map.get("user_defined_function_resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_defined_function_resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_disposition: {
                        let field_value = match fields_map.get("write_disposition") {
                            Some(value) => value,
                            None => bail!("Missing field 'write_disposition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
