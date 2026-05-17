#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionCloudSpanner {
    /// Cloud Spanner database in the form `project/instance/database'.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: String,
    /// Cloud Spanner database role for fine-grained access control. The Cloud Spanner admin should have provisioned the database role with appropriate permissions, such as `SELECT` and `INSERT`. Other users should only use roles provided by their Cloud Spanner admins. The database role name must start with a letter, and can only contain letters, numbers, and underscores. For more details, see https://cloud.google.com/spanner/docs/fgac-about.
    #[builder(into)]
    #[serde(rename = "databaseRole")]
    pub r#database_role: Option<String>,
    /// Allows setting max parallelism per query when executing on Spanner independent compute resources. If unspecified, default values of parallelism are chosen that are dependent on the Cloud Spanner instance configuration. `useParallelism` and `useDataBoost` must be set when setting max parallelism.
    #[builder(into)]
    #[serde(rename = "maxParallelism")]
    pub r#max_parallelism: Option<i32>,
    /// If set, the request will be executed via Spanner independent compute resources. `use_parallelism` must be set when using data boost.
    #[builder(into)]
    #[serde(rename = "useDataBoost")]
    pub r#use_data_boost: Option<bool>,
    /// If parallelism should be used when reading from Cloud Spanner.
    #[builder(into)]
    #[serde(rename = "useParallelism")]
    pub r#use_parallelism: Option<bool>,
    /// (Optional, Deprecated)
    /// If the serverless analytics service should be used to read data from Cloud Spanner. `useParallelism` must be set when using serverless analytics.
    /// 
    /// > **Warning:** `useServerlessAnalytics` is deprecated and will be removed in a future major release. Use `useDataBoost` instead.
    #[builder(into)]
    #[serde(rename = "useServerlessAnalytics")]
    pub r#use_serverless_analytics: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionCloudSpanner {
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
                "database".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database,
                )
                .await,
            );
            map.insert(
                "database_role".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_role,
                )
                .await,
            );
            map.insert(
                "max_parallelism".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_parallelism,
                )
                .await,
            );
            map.insert(
                "use_data_boost".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_data_boost,
                )
                .await,
            );
            map.insert(
                "use_parallelism".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_parallelism,
                )
                .await,
            );
            map.insert(
                "use_serverless_analytics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_serverless_analytics,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionCloudSpanner {
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
                    r#database: {
                        let field_value = match fields_map.get("database") {
                            Some(value) => value,
                            None => bail!("Missing field 'database' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_role: {
                        let field_value = match fields_map.get("database_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_parallelism: {
                        let field_value = match fields_map.get("max_parallelism") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_parallelism' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_data_boost: {
                        let field_value = match fields_map.get("use_data_boost") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_data_boost' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_parallelism: {
                        let field_value = match fields_map.get("use_parallelism") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_parallelism' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_serverless_analytics: {
                        let field_value = match fields_map.get("use_serverless_analytics") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_serverless_analytics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
