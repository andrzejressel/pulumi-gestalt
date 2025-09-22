#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsSlotValueElicitationSettingWaitAndContinueSpecification {
    /// Specifies whether the bot will wait for a user to respond.
    /// When this field is `false`, wait and continue responses for a slot aren't used.
    /// If the active field isn't specified, the default is `true`.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: Option<bool>,
    /// Response that Amazon Lex sends to indicate that the bot is ready to continue the conversation.
    /// See the `continue_response` argument reference below.
    #[builder(into)]
    #[serde(rename = "continueResponses")]
    pub r#continue_responses: Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationContinueResponse>>,
    /// Response that Amazon Lex sends periodically to the user to indicate that the bot is still waiting for input from the user.
    /// See the `still_waiting_response` argument reference below.
    #[builder(into)]
    #[serde(rename = "stillWaitingResponses")]
    pub r#still_waiting_responses: Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponse>>,
    /// Response that Amazon Lex sends to indicate that the bot is waiting for the conversation to continue.
    /// See the `waiting_response` argument reference below.
    #[builder(into)]
    #[serde(rename = "waitingResponses")]
    pub r#waiting_responses: Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationWaitingResponse>>,
}
