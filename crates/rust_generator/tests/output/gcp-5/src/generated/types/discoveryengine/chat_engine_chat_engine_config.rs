#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChatEngineChatEngineConfig {
    /// The configuration to generate the Dialogflow agent that is associated to this Engine.
    /// Exactly one of `agent_creation_config` or `dialogflow_agent_to_link` must be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "agentCreationConfig")]
    pub r#agent_creation_config: Option<Box<super::super::types::discoveryengine::ChatEngineChatEngineConfigAgentCreationConfig>>,
    /// The resource name of an existing Dialogflow agent to link to this Chat Engine. Format: `projects/<Project_ID>/locations/<Location_ID>/agents/<Agent_ID>`.
    /// Exactly one of `agent_creation_config` or `dialogflow_agent_to_link` must be set.
    #[builder(into)]
    #[serde(rename = "dialogflowAgentToLink")]
    pub r#dialogflow_agent_to_link: Option<String>,
}
