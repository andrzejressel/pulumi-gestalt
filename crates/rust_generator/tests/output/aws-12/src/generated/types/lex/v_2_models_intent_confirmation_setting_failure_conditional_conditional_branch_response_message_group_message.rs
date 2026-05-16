#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupMessage {
    /// Configuration block for a message in a custom format defined by the client application. See `custom_payload`.
    #[builder(into)]
    #[serde(rename = "customPayload")]
    pub r#custom_payload: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupMessageCustomPayload>>,
    /// Configuration block for a message that defines a response card that the client application can show to the user. See `image_response_card`.
    #[builder(into)]
    #[serde(rename = "imageResponseCard")]
    pub r#image_response_card: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupMessageImageResponseCard>>,
    /// Configuration block for a message in plain text format. See `plain_text_message`.
    #[builder(into)]
    #[serde(rename = "plainTextMessage")]
    pub r#plain_text_message: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupMessagePlainTextMessage>>,
    /// Configuration block for a message in Speech Synthesis Markup Language (SSML). See `ssml_message`.
    #[builder(into)]
    #[serde(rename = "ssmlMessage")]
    pub r#ssml_message: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupMessageSsmlMessage>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupMessage {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("custom_payload".to_string(), self.r#custom_payload.to_pulumi_value().await);
            map.insert("image_response_card".to_string(), self.r#image_response_card.to_pulumi_value().await);
            map.insert("plain_text_message".to_string(), self.r#plain_text_message.to_pulumi_value().await);
            map.insert("ssml_message".to_string(), self.r#ssml_message.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupMessage {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#custom_payload: {
                        let field_value = match fields_map.get("custom_payload") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_payload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupMessageCustomPayload>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#image_response_card: {
                        let field_value = match fields_map.get("image_response_card") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_response_card' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupMessageImageResponseCard>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#plain_text_message: {
                        let field_value = match fields_map.get("plain_text_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'plain_text_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupMessagePlainTextMessage>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ssml_message: {
                        let field_value = match fields_map.get("ssml_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssml_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupMessageSsmlMessage>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
