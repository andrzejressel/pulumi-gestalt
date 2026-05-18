#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KxClusterDatabase {
    /// Configuration details for the disk cache to increase performance reading from a KX database mounted to the cluster. See cache_configurations.
    #[builder(into)]
    #[serde(rename = "cacheConfigurations")]
    pub r#cache_configurations: Option<Vec<super::super::types::finspace::KxClusterDatabaseCacheConfiguration>>,
    /// A unique identifier of the changeset that is associated with the cluster.
    #[builder(into)]
    #[serde(rename = "changesetId")]
    pub r#changeset_id: Option<String>,
    /// Name of the KX database.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// The name of the dataview to be used for caching historical data on disk. You cannot update to a different dataview name once a cluster is created. Use `lifecycle` `ignore_changes` for database to prevent any undesirable behaviors.
    #[builder(into)]
    #[serde(rename = "dataviewName")]
    pub r#dataview_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KxClusterDatabase {
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
                    "cache_configurations",
                    &self.r#cache_configurations,
                ),
                to_pulumi_object_field(
                    "changeset_id",
                    &self.r#changeset_id,
                ),
                to_pulumi_object_field(
                    "database_name",
                    &self.r#database_name,
                ),
                to_pulumi_object_field(
                    "dataview_name",
                    &self.r#dataview_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KxClusterDatabase {
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
                    r#cache_configurations: {
                        let field_value = match fields_map.get("cache_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#changeset_id: {
                        let field_value = match fields_map.get("changeset_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'changeset_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_name: {
                        let field_value = match fields_map.get("database_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dataview_name: {
                        let field_value = match fields_map.get("dataview_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataview_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
