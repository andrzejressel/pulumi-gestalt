#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTargetCloudSqlTargetFilterDatabaseResourceReference {
    /// Required. Name of a database within the instance.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: String,
    /// Required. Name of a database resource, for example, a table within the database.
    #[builder(into)]
    #[serde(rename = "databaseResource")]
    pub r#database_resource: String,
    /// Required. The instance where this resource is located. For example: Cloud SQL instance ID.
    #[builder(into)]
    #[serde(rename = "instance")]
    pub r#instance: String,
    /// Required. If within a project-level config, then this must match the config's project ID.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigTargetCloudSqlTargetFilterDatabaseResourceReference {
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
                    "database",
                    &self.r#database,
                ),
                to_pulumi_object_field(
                    "database_resource",
                    &self.r#database_resource,
                ),
                to_pulumi_object_field(
                    "instance",
                    &self.r#instance,
                ),
                to_pulumi_object_field(
                    "project_id",
                    &self.r#project_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDiscoveryConfigTargetCloudSqlTargetFilterDatabaseResourceReference {
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
                    r#database_resource: {
                        let field_value = match fields_map.get("database_resource") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_resource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance: {
                        let field_value = match fields_map.get("instance") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project_id: {
                        let field_value = match fields_map.get("project_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'project_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
