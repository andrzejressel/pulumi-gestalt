#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAgentAgentVersionsAgentVersionSummary {
    /// Name of agent to which the version belongs.
    #[builder(into)]
    #[serde(rename = "agentName")]
    pub r#agent_name: String,
    /// Status of the agent to which the version belongs.
    #[builder(into)]
    #[serde(rename = "agentStatus")]
    pub r#agent_status: String,
    /// Version of the agent.
    #[builder(into)]
    #[serde(rename = "agentVersion")]
    pub r#agent_version: String,
    /// Time at which the version was created.
    #[builder(into)]
    #[serde(rename = "createdAt")]
    pub r#created_at: String,
    /// Description of the version of the agent.
    /// * `GuardrailConfiguration` - Details aout the guardrail associated with the agent. See Guardrail Configuration
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    #[builder(into)]
    #[serde(rename = "guardrailConfigurations")]
    pub r#guardrail_configurations: Option<Vec<super::super::types::bedrock::GetAgentAgentVersionsAgentVersionSummaryGuardrailConfiguration>>,
    /// Time at which the version was last updated.
    #[builder(into)]
    #[serde(rename = "updatedAt")]
    pub r#updated_at: String,
}
