#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryWorkflowConfigInvocationConfig {
    /// Optional. When set to true, any incremental tables will be fully refreshed.
    #[builder(into)]
    #[serde(rename = "fullyRefreshIncrementalTablesEnabled")]
    pub r#fully_refresh_incremental_tables_enabled: Option<bool>,
    /// Optional. The set of tags to include.
    #[builder(into)]
    #[serde(rename = "includedTags")]
    pub r#included_tags: Option<Vec<String>>,
    /// Optional. The set of action identifiers to include.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "includedTargets")]
    pub r#included_targets: Option<Vec<super::super::types::dataform::RepositoryWorkflowConfigInvocationConfigIncludedTarget>>,
    /// Optional. The service account to run workflow invocations under.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Option<String>,
    /// Optional. When set to true, transitive dependencies of included actions will be executed.
    #[builder(into)]
    #[serde(rename = "transitiveDependenciesIncluded")]
    pub r#transitive_dependencies_included: Option<bool>,
    /// Optional. When set to true, transitive dependents of included actions will be executed.
    #[builder(into)]
    #[serde(rename = "transitiveDependentsIncluded")]
    pub r#transitive_dependents_included: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RepositoryWorkflowConfigInvocationConfig {
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
                    "fully_refresh_incremental_tables_enabled",
                    &self.r#fully_refresh_incremental_tables_enabled,
                ),
                to_pulumi_object_field(
                    "included_tags",
                    &self.r#included_tags,
                ),
                to_pulumi_object_field(
                    "included_targets",
                    &self.r#included_targets,
                ),
                to_pulumi_object_field(
                    "service_account",
                    &self.r#service_account,
                ),
                to_pulumi_object_field(
                    "transitive_dependencies_included",
                    &self.r#transitive_dependencies_included,
                ),
                to_pulumi_object_field(
                    "transitive_dependents_included",
                    &self.r#transitive_dependents_included,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RepositoryWorkflowConfigInvocationConfig {
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
                    r#fully_refresh_incremental_tables_enabled: {
                        let field_value = match fields_map.get("fully_refresh_incremental_tables_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'fully_refresh_incremental_tables_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_tags: {
                        let field_value = match fields_map.get("included_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_targets: {
                        let field_value = match fields_map.get("included_targets") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_targets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account: {
                        let field_value = match fields_map.get("service_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transitive_dependencies_included: {
                        let field_value = match fields_map.get("transitive_dependencies_included") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitive_dependencies_included' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transitive_dependents_included: {
                        let field_value = match fields_map.get("transitive_dependents_included") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitive_dependents_included' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
