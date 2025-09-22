#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecification {
    /// Configuration block for conditional branches to evaluate after the dialog code hook throws an exception or returns with the State field of the Intent object set to Failed.
    #[builder(into)]
    #[serde(rename = "failureConditional")]
    pub r#failure_conditional: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationFailureConditional>>,
    /// Configuration block for the next step the bot runs after the dialog code hook throws an exception or returns with the State field of the Intent object set to Failed . See `failure_next_step`.
    #[builder(into)]
    #[serde(rename = "failureNextStep")]
    pub r#failure_next_step: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationFailureNextStep>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `failure_response`.
    #[builder(into)]
    #[serde(rename = "failureResponse")]
    pub r#failure_response: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationFailureResponse>>,
    /// Configuration block for conditional branches to evaluate after the dialog code hook finishes successfully. See `success_conditional`.
    #[builder(into)]
    #[serde(rename = "successConditional")]
    pub r#success_conditional: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationSuccessConditional>>,
    /// Configuration block for the next step the bot runs after the dialog code hook finishes successfully. See `success_next_step`.
    #[builder(into)]
    #[serde(rename = "successNextStep")]
    pub r#success_next_step: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationSuccessNextStep>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `success_response`.
    #[builder(into)]
    #[serde(rename = "successResponse")]
    pub r#success_response: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationSuccessResponse>>,
    /// Configuration block for conditional branches to evaluate if the code hook times out. See `timeout_conditional`.
    #[builder(into)]
    #[serde(rename = "timeoutConditional")]
    pub r#timeout_conditional: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationTimeoutConditional>>,
    /// Configuration block for the next step that the bot runs when the code hook times out. See `timeout_next_step`.
    #[builder(into)]
    #[serde(rename = "timeoutNextStep")]
    pub r#timeout_next_step: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationTimeoutNextStep>>,
    /// Configuration block for a list of message groups that Amazon Lex uses to respond the user input. See `timeout_response`.
    #[builder(into)]
    #[serde(rename = "timeoutResponse")]
    pub r#timeout_response: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationTimeoutResponse>>,
}
