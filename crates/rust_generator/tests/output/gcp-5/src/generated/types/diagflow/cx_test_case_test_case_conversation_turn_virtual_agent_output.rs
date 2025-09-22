#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxTestCaseTestCaseConversationTurnVirtualAgentOutput {
    /// The [Page](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows.pages#Page) on which the utterance was spoken.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "currentPage")]
    pub r#current_page: Option<Box<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnVirtualAgentOutputCurrentPage>>,
    /// The session parameters available to the bot at this point.
    #[builder(into)]
    #[serde(rename = "sessionParameters")]
    pub r#session_parameters: Option<String>,
    /// The text responses from the agent for the turn.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "textResponses")]
    pub r#text_responses: Option<Vec<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnVirtualAgentOutputTextResponse>>,
    /// The [Intent](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.intents#Intent) that triggered the response.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "triggeredIntent")]
    pub r#triggered_intent: Option<Box<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnVirtualAgentOutputTriggeredIntent>>,
}
