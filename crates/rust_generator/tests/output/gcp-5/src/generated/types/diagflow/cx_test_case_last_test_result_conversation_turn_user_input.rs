#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxTestCaseLastTestResultConversationTurnUserInput {
    /// Whether sentiment analysis is enabled.
    #[builder(into)]
    #[serde(rename = "enableSentimentAnalysis")]
    pub r#enable_sentiment_analysis: Option<bool>,
    /// Parameters that need to be injected into the conversation during intent detection.
    #[builder(into)]
    #[serde(rename = "injectedParameters")]
    pub r#injected_parameters: Option<String>,
    /// User input. Supports text input, event input, dtmf input in the test case.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "input")]
    pub r#input: Box<Option<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnUserInputInput>>,
    /// If webhooks should be allowed to trigger in response to the user utterance. Often if parameters are injected, webhooks should not be enabled.
    #[builder(into)]
    #[serde(rename = "isWebhookEnabled")]
    pub r#is_webhook_enabled: Option<bool>,
}
