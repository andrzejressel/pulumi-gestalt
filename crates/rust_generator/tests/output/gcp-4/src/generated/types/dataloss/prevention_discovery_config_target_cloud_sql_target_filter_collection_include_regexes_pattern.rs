#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTargetCloudSqlTargetFilterCollectionIncludeRegexesPattern {
    /// Regex to test the database name against. If empty, all databases match.
    #[builder(into)]
    #[serde(rename = "databaseRegex")]
    pub r#database_regex: Option<String>,
    /// Regex to test the database resource's name against. An example of a database resource name is a table's name. Other database resource names like view names could be included in the future. If empty, all database resources match.'
    #[builder(into)]
    #[serde(rename = "databaseResourceNameRegex")]
    pub r#database_resource_name_regex: Option<String>,
    /// Regex to test the instance name against. If empty, all instances match.
    #[builder(into)]
    #[serde(rename = "instanceRegex")]
    pub r#instance_regex: Option<String>,
    /// For organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project.
    #[builder(into)]
    #[serde(rename = "projectIdRegex")]
    pub r#project_id_regex: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigTargetCloudSqlTargetFilterCollectionIncludeRegexesPattern {
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
                    "database_regex",
                    &self.r#database_regex,
                ),
                to_pulumi_object_field(
                    "database_resource_name_regex",
                    &self.r#database_resource_name_regex,
                ),
                to_pulumi_object_field(
                    "instance_regex",
                    &self.r#instance_regex,
                ),
                to_pulumi_object_field(
                    "project_id_regex",
                    &self.r#project_id_regex,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDiscoveryConfigTargetCloudSqlTargetFilterCollectionIncludeRegexesPattern {
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
                    r#database_regex: {
                        let field_value = match fields_map.get("database_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_resource_name_regex: {
                        let field_value = match fields_map.get("database_resource_name_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_resource_name_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_regex: {
                        let field_value = match fields_map.get("instance_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project_id_regex: {
                        let field_value = match fields_map.get("project_id_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'project_id_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
