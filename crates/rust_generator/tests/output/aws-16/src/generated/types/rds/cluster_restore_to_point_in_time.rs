#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterRestoreToPointInTime {
    /// Date and time in UTC format to restore the database cluster to. Conflicts with `use_latest_restorable_time`.
    #[builder(into)]
    #[serde(rename = "restoreToTime")]
    pub r#restore_to_time: Option<String>,
    /// Type of restore to be performed.
    /// Valid options are `full-copy` (default) and `copy-on-write`.
    #[builder(into)]
    #[serde(rename = "restoreType")]
    pub r#restore_type: Option<String>,
    /// Identifier of the source database cluster from which to restore. When restoring from a cluster in another AWS account, the identifier is the ARN of that cluster.
    #[builder(into)]
    #[serde(rename = "sourceClusterIdentifier")]
    pub r#source_cluster_identifier: Option<String>,
    /// Cluster resource ID of the source database cluster from which to restore. To be used for restoring a deleted cluster in the same account which still has a retained automatic backup available.
    #[builder(into)]
    #[serde(rename = "sourceClusterResourceId")]
    pub r#source_cluster_resource_id: Option<String>,
    /// Set to true to restore the database cluster to the latest restorable backup time. Defaults to false. Conflicts with `restore_to_time`.
    #[builder(into)]
    #[serde(rename = "useLatestRestorableTime")]
    pub r#use_latest_restorable_time: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterRestoreToPointInTime {
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
                    "restore_to_time",
                    &self.r#restore_to_time,
                ),
                to_pulumi_object_field(
                    "restore_type",
                    &self.r#restore_type,
                ),
                to_pulumi_object_field(
                    "source_cluster_identifier",
                    &self.r#source_cluster_identifier,
                ),
                to_pulumi_object_field(
                    "source_cluster_resource_id",
                    &self.r#source_cluster_resource_id,
                ),
                to_pulumi_object_field(
                    "use_latest_restorable_time",
                    &self.r#use_latest_restorable_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterRestoreToPointInTime {
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
                    r#restore_to_time: {
                        let field_value = match fields_map.get("restore_to_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'restore_to_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restore_type: {
                        let field_value = match fields_map.get("restore_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'restore_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_cluster_identifier: {
                        let field_value = match fields_map.get("source_cluster_identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_cluster_identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_cluster_resource_id: {
                        let field_value = match fields_map.get("source_cluster_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_cluster_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_latest_restorable_time: {
                        let field_value = match fields_map.get("use_latest_restorable_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_latest_restorable_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
