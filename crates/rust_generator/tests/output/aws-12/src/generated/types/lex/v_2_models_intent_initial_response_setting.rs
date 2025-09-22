#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentInitialResponseSetting {
    /// Configuration block for the dialog code hook that is called by Amazon Lex at a step of the conversation. See `code_hook`.
    #[builder(into)]
    #[serde(rename = "codeHook")]
    pub r#code_hook: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHook>>,
    /// Configuration block for conditional branches. Branches are evaluated in the order that they are entered in the list. The first branch with a condition that evaluates to true is executed. The last branch in the list is the default branch. The default branch should not have any condition expression. The default branch is executed if no other branch has a matching condition. See `conditional`.
    #[builder(into)]
    #[serde(rename = "conditional")]
    pub r#conditional: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingConditional>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `initial_response`.
    #[builder(into)]
    #[serde(rename = "initialResponse")]
    pub r#initial_response: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingInitialResponse>>,
    /// Configuration block for the next step in the conversation. See `next_step`.
    #[builder(into)]
    #[serde(rename = "nextStep")]
    pub r#next_step: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingNextStep>>,
}
