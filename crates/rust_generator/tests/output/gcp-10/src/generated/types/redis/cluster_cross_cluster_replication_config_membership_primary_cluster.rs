#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterCrossClusterReplicationConfigMembershipPrimaryCluster {
    /// The full resource path of the primary cluster in the format: projects/{project}/locations/{region}/clusters/{cluster-id}
    #[builder(into)]
    #[serde(rename = "cluster")]
    pub r#cluster: Option<String>,
    /// (Output)
    /// The unique id of the primary cluster.
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: Option<String>,
}
