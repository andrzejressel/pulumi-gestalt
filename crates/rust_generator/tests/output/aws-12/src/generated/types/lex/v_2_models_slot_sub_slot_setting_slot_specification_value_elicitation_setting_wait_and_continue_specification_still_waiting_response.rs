#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponse {
    /// Whether the user can interrupt a speech response from Amazon Lex.
    #[builder(into)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Option<bool>,
    /// How often a message should be sent to the user.
    #[builder(into)]
    #[serde(rename = "frequencyInSeconds")]
    pub r#frequency_in_seconds: i32,
    #[builder(into)]
    #[serde(rename = "messageGroups")]
    pub r#message_groups: Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponseMessageGroup>>,
    /// If Amazon Lex waits longer than this length of time for a response, it will stop sending messages.
    #[builder(into)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: i32,
}
