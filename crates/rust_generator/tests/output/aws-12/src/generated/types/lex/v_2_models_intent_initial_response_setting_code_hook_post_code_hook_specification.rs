#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecification {
    /// Configuration block for conditional branches to evaluate after the dialog code hook throws an exception or returns with the State field of the Intent object set to Failed.
    #[builder(into, default)]
    #[serde(rename = "failureConditional")]
    pub r#failure_conditional: Box<Option<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationFailureConditional>>,
    /// Configuration block for the next step the bot runs after the dialog code hook throws an exception or returns with the State field of the Intent object set to Failed . See `failure_next_step`.
    #[builder(into, default)]
    #[serde(rename = "failureNextStep")]
    pub r#failure_next_step: Box<Option<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationFailureNextStep>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `failure_response`.
    #[builder(into, default)]
    #[serde(rename = "failureResponse")]
    pub r#failure_response: Box<Option<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationFailureResponse>>,
    /// Configuration block for conditional branches to evaluate after the dialog code hook finishes successfully. See `success_conditional`.
    #[builder(into, default)]
    #[serde(rename = "successConditional")]
    pub r#success_conditional: Box<Option<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationSuccessConditional>>,
    /// Configuration block for the next step the bot runs after the dialog code hook finishes successfully. See `success_next_step`.
    #[builder(into, default)]
    #[serde(rename = "successNextStep")]
    pub r#success_next_step: Box<Option<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationSuccessNextStep>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `success_response`.
    #[builder(into, default)]
    #[serde(rename = "successResponse")]
    pub r#success_response: Box<Option<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationSuccessResponse>>,
    /// Configuration block for conditional branches to evaluate if the code hook times out. See `timeout_conditional`.
    #[builder(into, default)]
    #[serde(rename = "timeoutConditional")]
    pub r#timeout_conditional: Box<Option<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationTimeoutConditional>>,
    /// Configuration block for the next step that the bot runs when the code hook times out. See `timeout_next_step`.
    #[builder(into, default)]
    #[serde(rename = "timeoutNextStep")]
    pub r#timeout_next_step: Box<Option<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationTimeoutNextStep>>,
    /// Configuration block for a list of message groups that Amazon Lex uses to respond the user input. See `timeout_response`.
    #[builder(into, default)]
    #[serde(rename = "timeoutResponse")]
    pub r#timeout_response: Box<Option<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationTimeoutResponse>>,
}
