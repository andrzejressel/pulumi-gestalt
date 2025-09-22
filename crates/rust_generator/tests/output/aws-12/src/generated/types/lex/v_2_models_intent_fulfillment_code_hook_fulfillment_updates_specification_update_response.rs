#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecificationUpdateResponse {
    /// Whether the user can interrupt the start message while it is playing.
    #[builder(into)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Option<bool>,
    /// Frequency that a message is sent to the user. When the period ends, Amazon Lex chooses a message from the message groups and plays it to the user. If the fulfillment Lambda returns before the first period ends, an update message is not played to the user.
    #[builder(into)]
    #[serde(rename = "frequencyInSeconds")]
    pub r#frequency_in_seconds: i32,
    /// Between 1-5 configuration block message groups that contain start messages. Amazon Lex chooses one of the messages to play to the user. See `message_group`.
    #[builder(into)]
    #[serde(rename = "messageGroups")]
    pub r#message_groups: Option<Vec<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecificationUpdateResponseMessageGroup>>,
}
