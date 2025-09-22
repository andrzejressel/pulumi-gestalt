#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KeyspaceReplicationSpecification {
    /// Replication regions. If `replication_strategy` is `MULTI_REGION`, `region_list` requires the current Region and at least one additional AWS Region where the keyspace is going to be replicated in.
    #[builder(into)]
    #[serde(rename = "regionLists")]
    pub r#region_lists: Option<Vec<String>>,
    /// Replication strategy. Valid values: `SINGLE_REGION` and `MULTI_REGION`.
    #[builder(into)]
    #[serde(rename = "replicationStrategy")]
    pub r#replication_strategy: Option<String>,
}
