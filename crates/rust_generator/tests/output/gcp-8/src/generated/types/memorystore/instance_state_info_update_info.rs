#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceStateInfoUpdateInfo {
    /// (Output)
    /// Output only. Target number of replica nodes per shard for the instance.
    #[builder(into)]
    #[serde(rename = "targetReplicaCount")]
    pub r#target_replica_count: Option<i32>,
    /// (Output)
    /// Output only. Target number of shards for the instance.
    #[builder(into)]
    #[serde(rename = "targetShardCount")]
    pub r#target_shard_count: Option<i32>,
}
