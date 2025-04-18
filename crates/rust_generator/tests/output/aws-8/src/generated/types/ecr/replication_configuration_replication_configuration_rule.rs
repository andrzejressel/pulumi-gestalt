#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReplicationConfigurationReplicationConfigurationRule {
    /// the details of a replication destination. A maximum of 25 are allowed per `rule`. See Destination.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Box<Vec<super::super::types::ecr::ReplicationConfigurationReplicationConfigurationRuleDestination>>,
    /// filters for a replication rule. See Repository Filter.
    #[builder(into, default)]
    #[serde(rename = "repositoryFilters")]
    pub r#repository_filters: Box<Option<Vec<super::super::types::ecr::ReplicationConfigurationReplicationConfigurationRuleRepositoryFilter>>>,
}
