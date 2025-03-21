#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BotClarificationPrompt {
    /// The number of times to prompt the user for information.
    #[builder(into)]
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: Box<i32>,
    #[builder(into)]
    #[serde(rename = "messages")]
    pub r#messages: Box<Vec<super::super::types::lex::BotClarificationPromptMessage>>,
    #[builder(into, default)]
    #[serde(rename = "responseCard")]
    pub r#response_card: Box<Option<String>>,
}
