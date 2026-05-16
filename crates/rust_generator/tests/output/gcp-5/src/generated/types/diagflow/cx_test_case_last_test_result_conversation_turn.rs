#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxTestCaseLastTestResultConversationTurn {
    /// The user input.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "userInput")]
    pub r#user_input: Option<Box<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnUserInput>>,
    /// The virtual agent output.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "virtualAgentOutput")]
    pub r#virtual_agent_output: Option<Box<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnVirtualAgentOutput>>,
}
