#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxPageTransitionRouteTriggerFulfillment {
    /// Conditional cases for this fulfillment.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditionalCases")]
    pub r#conditional_cases: Option<Vec<super::super::types::diagflow::CxPageTransitionRouteTriggerFulfillmentConditionalCase>>,
    /// The list of rich message responses to present to the user.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "messages")]
    pub r#messages: Option<Vec<super::super::types::diagflow::CxPageTransitionRouteTriggerFulfillmentMessage>>,
    /// Whether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks.
    #[builder(into)]
    #[serde(rename = "returnPartialResponses")]
    pub r#return_partial_responses: Option<bool>,
    /// Set parameter values before executing the webhook.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "setParameterActions")]
    pub r#set_parameter_actions: Option<Vec<super::super::types::diagflow::CxPageTransitionRouteTriggerFulfillmentSetParameterAction>>,
    /// The tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Option<String>,
    /// The webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>.
    #[builder(into)]
    #[serde(rename = "webhook")]
    pub r#webhook: Option<String>,
}
