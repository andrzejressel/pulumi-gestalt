#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryWorkspaceCompilationOverrides {
    /// The default database (Google Cloud project ID).
    #[builder(into)]
    #[serde(rename = "defaultDatabase")]
    pub r#default_database: Option<String>,
    /// The suffix that should be appended to all schema (BigQuery dataset ID) names.
    #[builder(into)]
    #[serde(rename = "schemaSuffix")]
    pub r#schema_suffix: Option<String>,
    /// The prefix that should be prepended to all table names.
    #[builder(into)]
    #[serde(rename = "tablePrefix")]
    pub r#table_prefix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RepositoryWorkspaceCompilationOverrides {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "default_database".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_database,
                )
                .await,
            );
            map.insert(
                "schema_suffix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_suffix,
                )
                .await,
            );
            map.insert(
                "table_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#table_prefix,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RepositoryWorkspaceCompilationOverrides {
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
                    r#default_database: {
                        let field_value = match fields_map.get("default_database") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_database' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
