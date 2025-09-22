#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReplicatorReplicationInfoListTopicReplication {
    /// Whether to periodically configure remote topic ACLs to match their corresponding upstream topics.
    #[builder(into)]
    #[serde(rename = "copyAccessControlListsForTopics")]
    pub r#copy_access_control_lists_for_topics: Option<bool>,
    /// Whether to periodically configure remote topics to match their corresponding upstream topics.
    #[builder(into)]
    #[serde(rename = "copyTopicConfigurations")]
    pub r#copy_topic_configurations: Option<bool>,
    /// Whether to periodically check for new topics and partitions.
    #[builder(into)]
    #[serde(rename = "detectAndCopyNewTopics")]
    pub r#detect_and_copy_new_topics: Option<bool>,
    /// Configuration for specifying the position in the topics to start replicating from.
    #[builder(into)]
    #[serde(rename = "startingPosition")]
    pub r#starting_position: Option<Box<super::super::types::msk::ReplicatorReplicationInfoListTopicReplicationStartingPosition>>,
    #[builder(into)]
    #[serde(rename = "topicNameConfiguration")]
    pub r#topic_name_configuration: Option<Box<super::super::types::msk::ReplicatorReplicationInfoListTopicReplicationTopicNameConfiguration>>,
    /// List of regular expression patterns indicating the topics that should not be replica.
    #[builder(into)]
    #[serde(rename = "topicsToExcludes")]
    pub r#topics_to_excludes: Option<Vec<String>>,
    /// List of regular expression patterns indicating the topics to copy.
    #[builder(into)]
    #[serde(rename = "topicsToReplicates")]
    pub r#topics_to_replicates: Vec<String>,
}
