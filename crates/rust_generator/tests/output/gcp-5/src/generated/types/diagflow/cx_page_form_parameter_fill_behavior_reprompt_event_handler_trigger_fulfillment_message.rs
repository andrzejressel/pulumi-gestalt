#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxPageFormParameterFillBehaviorRepromptEventHandlerTriggerFulfillmentMessage {
    /// The channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned.
    #[builder(into)]
    #[serde(rename = "channel")]
    pub r#channel: Option<String>,
    /// Indicates that the conversation succeeded, i.e., the bot handled the issue that the customer talked to it about.
    /// Dialogflow only uses this to determine which conversations should be counted as successful and doesn't process the metadata in this message in any way. Note that Dialogflow also considers conversations that get to the conversation end page as successful even if they don't return ConversationSuccess.
    /// You may set this, for example:
    /// * In the entryFulfillment of a Page if entering the page indicates that the conversation succeeded.
    /// * In a webhook response when you determine that you handled the customer issue.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conversationSuccess")]
    pub r#conversation_success: Option<Box<super::super::types::diagflow::CxPageFormParameterFillBehaviorRepromptEventHandlerTriggerFulfillmentMessageConversationSuccess>>,
    /// Indicates that the conversation should be handed off to a live agent.
    /// Dialogflow only uses this to determine which conversations were handed off to a human agent for measurement purposes. What else to do with this signal is up to you and your handoff procedures.
    /// You may set this, for example:
    /// * In the entryFulfillment of a Page if entering the page indicates something went extremely wrong in the conversation.
    /// * In a webhook response when you determine that the customer issue can only be handled by a human.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "liveAgentHandoff")]
    pub r#live_agent_handoff: Option<Box<super::super::types::diagflow::CxPageFormParameterFillBehaviorRepromptEventHandlerTriggerFulfillmentMessageLiveAgentHandoff>>,
    /// A text or ssml response that is preferentially used for TTS output audio synthesis, as described in the comment on the ResponseMessage message.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "outputAudioText")]
    pub r#output_audio_text: Option<Box<super::super::types::diagflow::CxPageFormParameterFillBehaviorRepromptEventHandlerTriggerFulfillmentMessageOutputAudioText>>,
    /// A custom, platform-specific payload.
    #[builder(into)]
    #[serde(rename = "payload")]
    pub r#payload: Option<String>,
    /// Specifies an audio clip to be played by the client as part of the response.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "playAudio")]
    pub r#play_audio: Option<Box<super::super::types::diagflow::CxPageFormParameterFillBehaviorRepromptEventHandlerTriggerFulfillmentMessagePlayAudio>>,
    /// Represents the signal that telles the client to transfer the phone call connected to the agent to a third-party endpoint.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "telephonyTransferCall")]
    pub r#telephony_transfer_call: Option<Box<super::super::types::diagflow::CxPageFormParameterFillBehaviorRepromptEventHandlerTriggerFulfillmentMessageTelephonyTransferCall>>,
    /// The text response message.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "text")]
    pub r#text: Option<Box<super::super::types::diagflow::CxPageFormParameterFillBehaviorRepromptEventHandlerTriggerFulfillmentMessageText>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxPageFormParameterFillBehaviorRepromptEventHandlerTriggerFulfillmentMessage {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "channel",
                    &self.r#channel,
                ),
                to_pulumi_object_field(
                    "conversation_success",
                    &self.r#conversation_success,
                ),
                to_pulumi_object_field(
                    "live_agent_handoff",
                    &self.r#live_agent_handoff,
                ),
                to_pulumi_object_field(
                    "output_audio_text",
                    &self.r#output_audio_text,
                ),
                to_pulumi_object_field(
                    "payload",
                    &self.r#payload,
                ),
                to_pulumi_object_field(
                    "play_audio",
                    &self.r#play_audio,
                ),
                to_pulumi_object_field(
                    "telephony_transfer_call",
                    &self.r#telephony_transfer_call,
                ),
                to_pulumi_object_field(
                    "text",
                    &self.r#text,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxPageFormParameterFillBehaviorRepromptEventHandlerTriggerFulfillmentMessage {
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
                    r#channel: {
                        let field_value = match fields_map.get("channel") {
                            Some(value) => value,
                            None => bail!("Missing field 'channel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#conversation_success: {
                        let field_value = match fields_map.get("conversation_success") {
                            Some(value) => value,
                            None => bail!("Missing field 'conversation_success' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#live_agent_handoff: {
                        let field_value = match fields_map.get("live_agent_handoff") {
                            Some(value) => value,
                            None => bail!("Missing field 'live_agent_handoff' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_audio_text: {
                        let field_value = match fields_map.get("output_audio_text") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_audio_text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#payload: {
                        let field_value = match fields_map.get("payload") {
                            Some(value) => value,
                            None => bail!("Missing field 'payload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#play_audio: {
                        let field_value = match fields_map.get("play_audio") {
                            Some(value) => value,
                            None => bail!("Missing field 'play_audio' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#telephony_transfer_call: {
                        let field_value = match fields_map.get("telephony_transfer_call") {
                            Some(value) => value,
                            None => bail!("Missing field 'telephony_transfer_call' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text: {
                        let field_value = match fields_map.get("text") {
                            Some(value) => value,
                            None => bail!("Missing field 'text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
