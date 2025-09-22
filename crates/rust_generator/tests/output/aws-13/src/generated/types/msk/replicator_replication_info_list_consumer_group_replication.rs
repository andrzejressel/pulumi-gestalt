#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReplicatorReplicationInfoListConsumerGroupReplication {
    /// List of regular expression patterns indicating the consumer groups that should not be replicated.
    #[builder(into)]
    #[serde(rename = "consumerGroupsToExcludes")]
    pub r#consumer_groups_to_excludes: Option<Vec<String>>,
    /// List of regular expression patterns indicating the consumer groups to copy.
    #[builder(into)]
    #[serde(rename = "consumerGroupsToReplicates")]
    pub r#consumer_groups_to_replicates: Vec<String>,
    /// Whether to periodically check for new consumer groups.
    #[builder(into)]
    #[serde(rename = "detectAndCopyNewConsumerGroups")]
    pub r#detect_and_copy_new_consumer_groups: Option<bool>,
    /// Whether to periodically write the translated offsets to __consumer_offsets topic in target cluster.
    #[builder(into)]
    #[serde(rename = "synchroniseConsumerGroupOffsets")]
    pub r#synchronise_consumer_group_offsets: Option<bool>,
}
