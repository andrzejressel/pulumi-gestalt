#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterCrossClusterReplicationConfig {
    /// The role of the cluster in cross cluster replication. Supported values are:
    /// 1. `CLUSTER_ROLE_UNSPECIFIED`: This is an independent cluster that has never participated in cross cluster replication. It allows both reads and writes.
    /// 1. `NONE`: This is an independent cluster that previously participated in cross cluster replication(either as a `PRIMARY` or `SECONDARY` cluster). It allows both reads and writes.
    /// 1. `PRIMARY`: This cluster serves as the replication source for secondary clusters that are replicating from it. Any data written to it is automatically replicated to its secondary clusters. It allows both reads and writes.
    /// 1. `SECONDARY`: This cluster replicates data from the primary cluster. It allows only reads.
    /// Possible values are: `CLUSTER_ROLE_UNSPECIFIED`, `NONE`, `PRIMARY`, `SECONDARY`.
    #[builder(into)]
    #[serde(rename = "clusterRole")]
    pub r#cluster_role: Option<String>,
    /// (Output)
    /// An output only view of all the member clusters participating in cross cluster replication. This field is populated for all the member clusters irrespective of their cluster role.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "memberships")]
    pub r#memberships: Option<Vec<super::super::types::redis::ClusterCrossClusterReplicationConfigMembership>>,
    /// Details of the primary cluster that is used as the replication source for this secondary cluster. This is allowed to be set only for clusters whose cluster role is of type `SECONDARY`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "primaryCluster")]
    pub r#primary_cluster: Option<Box<super::super::types::redis::ClusterCrossClusterReplicationConfigPrimaryCluster>>,
    /// List of secondary clusters that are replicating from this primary cluster. This is allowed to be set only for clusters whose cluster role is of type `PRIMARY`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secondaryClusters")]
    pub r#secondary_clusters: Option<Vec<super::super::types::redis::ClusterCrossClusterReplicationConfigSecondaryCluster>>,
    /// (Output)
    /// The last time cross cluster replication config was updated.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterCrossClusterReplicationConfig {
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
                    "cluster_role",
                    &self.r#cluster_role,
                ),
                to_pulumi_object_field(
                    "memberships",
                    &self.r#memberships,
                ),
                to_pulumi_object_field(
                    "primary_cluster",
                    &self.r#primary_cluster,
                ),
                to_pulumi_object_field(
                    "secondary_clusters",
                    &self.r#secondary_clusters,
                ),
                to_pulumi_object_field(
                    "update_time",
                    &self.r#update_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterCrossClusterReplicationConfig {
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
                    r#cluster_role: {
                        let field_value = match fields_map.get("cluster_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memberships: {
                        let field_value = match fields_map.get("memberships") {
                            Some(value) => value,
                            None => bail!("Missing field 'memberships' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_cluster: {
                        let field_value = match fields_map.get("primary_cluster") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_cluster' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_clusters: {
                        let field_value = match fields_map.get("secondary_clusters") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_clusters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#update_time: {
                        let field_value = match fields_map.get("update_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'update_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
