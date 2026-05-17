#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AttachedDatabaseConfigurationSharing {
    /// List of external tables exclude from the follower database.
    #[builder(into)]
    #[serde(rename = "externalTablesToExcludes")]
    pub r#external_tables_to_excludes: Option<Vec<String>>,
    /// List of external tables to include in the follower database.
    #[builder(into)]
    #[serde(rename = "externalTablesToIncludes")]
    pub r#external_tables_to_includes: Option<Vec<String>>,
    /// List of materialized views exclude from the follower database.
    #[builder(into)]
    #[serde(rename = "materializedViewsToExcludes")]
    pub r#materialized_views_to_excludes: Option<Vec<String>>,
    /// List of materialized views to include in the follower database.
    #[builder(into)]
    #[serde(rename = "materializedViewsToIncludes")]
    pub r#materialized_views_to_includes: Option<Vec<String>>,
    /// List of tables to exclude from the follower database.
    #[builder(into)]
    #[serde(rename = "tablesToExcludes")]
    pub r#tables_to_excludes: Option<Vec<String>>,
    /// List of tables to include in the follower database.
    #[builder(into)]
    #[serde(rename = "tablesToIncludes")]
    pub r#tables_to_includes: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AttachedDatabaseConfigurationSharing {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "external_tables_to_excludes",
                    &self.r#external_tables_to_excludes,
                ),
                to_pulumi_object_field(
                    "external_tables_to_includes",
                    &self.r#external_tables_to_includes,
                ),
                to_pulumi_object_field(
                    "materialized_views_to_excludes",
                    &self.r#materialized_views_to_excludes,
                ),
                to_pulumi_object_field(
                    "materialized_views_to_includes",
                    &self.r#materialized_views_to_includes,
                ),
                to_pulumi_object_field(
                    "tables_to_excludes",
                    &self.r#tables_to_excludes,
                ),
                to_pulumi_object_field(
                    "tables_to_includes",
                    &self.r#tables_to_includes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AttachedDatabaseConfigurationSharing {
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
                    r#external_tables_to_excludes: {
                        let field_value = match fields_map.get("external_tables_to_excludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_tables_to_excludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_tables_to_includes: {
                        let field_value = match fields_map.get("external_tables_to_includes") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_tables_to_includes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#materialized_views_to_excludes: {
                        let field_value = match fields_map.get("materialized_views_to_excludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'materialized_views_to_excludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#materialized_views_to_includes: {
                        let field_value = match fields_map.get("materialized_views_to_includes") {
                            Some(value) => value,
                            None => bail!("Missing field 'materialized_views_to_includes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tables_to_excludes: {
                        let field_value = match fields_map.get("tables_to_excludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'tables_to_excludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tables_to_includes: {
                        let field_value = match fields_map.get("tables_to_includes") {
                            Some(value) => value,
                            None => bail!("Missing field 'tables_to_includes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
