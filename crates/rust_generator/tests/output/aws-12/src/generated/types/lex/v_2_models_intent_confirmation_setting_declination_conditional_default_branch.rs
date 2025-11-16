#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSettingDeclinationConditionalDefaultBranch {
    /// Configuration block for the next step in the conversation. See `next_step`.
    #[builder(into)]
    #[serde(rename = "nextStep")]
    pub r#next_step: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationConditionalDefaultBranchNextStep>>,
    /// Configuration block for a list of message groups that Amazon Lex uses to respond to the user input. See `response`.
    #[builder(into)]
    #[serde(rename = "response")]
    pub r#response: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationConditionalDefaultBranchResponse>>,
}
