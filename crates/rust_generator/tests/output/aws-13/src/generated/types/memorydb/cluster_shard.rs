#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterShard {
    /// Name of the cluster. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Set of nodes in this shard.
    #[builder(into)]
    #[serde(rename = "nodes")]
    pub r#nodes: Option<Vec<super::super::types::memorydb::ClusterShardNode>>,
    /// Number of individual nodes in this shard.
    #[builder(into)]
    #[serde(rename = "numNodes")]
    pub r#num_nodes: Option<i32>,
    /// Keyspace for this shard. Example: `0-16383`.
    #[builder(into)]
    #[serde(rename = "slots")]
    pub r#slots: Option<String>,
}
