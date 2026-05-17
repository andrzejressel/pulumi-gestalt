#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RepositoryReleaseConfigCodeCompilationConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "assertion_schema",
                    &self.r#assertion_schema,
                ),
                to_pulumi_object_field(
                    "database_suffix",
                    &self.r#database_suffix,
                ),
                to_pulumi_object_field(
                    "default_database",
                    &self.r#default_database,
                ),
                to_pulumi_object_field(
                    "default_location",
                    &self.r#default_location,
                ),
                to_pulumi_object_field(
                    "default_schema",
                    &self.r#default_schema,
                ),
                to_pulumi_object_field(
                    "schema_suffix",
                    &self.r#schema_suffix,
                ),
                to_pulumi_object_field(
                    "table_prefix",
                    &self.r#table_prefix,
                ),
                to_pulumi_object_field(
                    "vars",
                    &self.r#vars,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RepositoryReleaseConfigCodeCompilationConfig {
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
                    r#assertion_schema: {
                        let field_value = match fields_map.get("assertion_schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'assertion_schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_suffix: {
                        let field_value = match fields_map.get("database_suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_database: {
                        let field_value = match fields_map.get("default_database") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_database' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_location: {
                        let field_value = match fields_map.get("default_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_schema: {
                        let field_value = match fields_map.get("default_schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_suffix: {
                        let field_value = match fields_map.get("schema_suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_prefix: {
                        let field_value = match fields_map.get("table_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vars: {
                        let field_value = match fields_map.get("vars") {
                            Some(value) => value,
                            None => bail!("Missing field 'vars' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
