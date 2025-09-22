#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
