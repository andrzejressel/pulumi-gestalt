#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicationConfigurationReplicationConfiguration {
    /// The replication rules for a replication configuration. A maximum of 10 are allowed per `replication_configuration`. See Rule
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Vec<super::super::types::ecr::ReplicationConfigurationReplicationConfigurationRule>,
}
