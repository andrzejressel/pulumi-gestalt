#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationSuccessConditionalDefaultBranchResponse {
    /// Whether the user can interrupt a speech response from Amazon Lex.
    #[builder(into)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Option<bool>,
    /// Configuration blocks for responses that Amazon Lex can send to the user. Amazon Lex chooses the actual response to send at runtime. See `message_group`.
    #[builder(into)]
    #[serde(rename = "messageGroups")]
    pub r#message_groups: Option<Vec<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationSuccessConditionalDefaultBranchResponseMessageGroup>>,
}
