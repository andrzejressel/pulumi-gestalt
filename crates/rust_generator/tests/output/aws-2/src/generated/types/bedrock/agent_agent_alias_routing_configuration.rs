#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentAgentAliasRoutingConfiguration {
    /// Version of the agent with which the alias is associated.
    #[builder(into)]
    #[serde(rename = "agentVersion")]
    pub r#agent_version: String,
    /// ARN of the Provisioned Throughput assigned to the agent alias.
    #[builder(into)]
    #[serde(rename = "provisionedThroughput")]
    pub r#provisioned_throughput: String,
}
