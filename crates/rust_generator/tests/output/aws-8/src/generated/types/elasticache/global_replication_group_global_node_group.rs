#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GlobalReplicationGroupGlobalNodeGroup {
    /// The ID of the global node group.
    #[builder(into)]
    #[serde(rename = "globalNodeGroupId")]
    pub r#global_node_group_id: Option<String>,
    /// The keyspace for this node group.
    #[builder(into)]
    #[serde(rename = "slots")]
    pub r#slots: Option<String>,
}
