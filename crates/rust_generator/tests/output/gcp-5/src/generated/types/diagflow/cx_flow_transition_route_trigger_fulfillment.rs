#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxFlowTransitionRouteTriggerFulfillment {
    /// Conditional cases for this fulfillment.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditionalCases")]
    pub r#conditional_cases: Option<Vec<super::super::types::diagflow::CxFlowTransitionRouteTriggerFulfillmentConditionalCase>>,
    /// The list of rich message responses to present to the user.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "messages")]
    pub r#messages: Option<Vec<super::super::types::diagflow::CxFlowTransitionRouteTriggerFulfillmentMessage>>,
    /// Whether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks.
    #[builder(into)]
    #[serde(rename = "returnPartialResponses")]
    pub r#return_partial_responses: Option<bool>,
    /// Set parameter values before executing the webhook.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "setParameterActions")]
    pub r#set_parameter_actions: Option<Vec<super::super::types::diagflow::CxFlowTransitionRouteTriggerFulfillmentSetParameterAction>>,
    /// The tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Option<String>,
    /// The webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>.
    #[builder(into)]
    #[serde(rename = "webhook")]
    pub r#webhook: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxFlowTransitionRouteTriggerFulfillment {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "conditional_cases",
                    &self.r#conditional_cases,
                ),
                to_pulumi_object_field(
                    "messages",
                    &self.r#messages,
                ),
                to_pulumi_object_field(
                    "return_partial_responses",
                    &self.r#return_partial_responses,
                ),
                to_pulumi_object_field(
                    "set_parameter_actions",
                    &self.r#set_parameter_actions,
                ),
                to_pulumi_object_field(
                    "tag",
                    &self.r#tag,
                ),
                to_pulumi_object_field(
                    "webhook",
                    &self.r#webhook,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxFlowTransitionRouteTriggerFulfillment {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#conditional_cases: {
                        let field_value = match fields_map.get("conditional_cases") {
                            Some(value) => value,
                            None => bail!("Missing field 'conditional_cases' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#messages: {
                        let field_value = match fields_map.get("messages") {
                            Some(value) => value,
                            None => bail!("Missing field 'messages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#return_partial_responses: {
                        let field_value = match fields_map.get("return_partial_responses") {
                            Some(value) => value,
                            None => bail!("Missing field 'return_partial_responses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#set_parameter_actions: {
                        let field_value = match fields_map.get("set_parameter_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'set_parameter_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag: {
                        let field_value = match fields_map.get("tag") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#webhook: {
                        let field_value = match fields_map.get("webhook") {
                            Some(value) => value,
                            None => bail!("Missing field 'webhook' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
