#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecification {
    /// Whether the user can interrupt a speech prompt attempt from the bot.
    #[builder(into)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Option<bool>,
    /// Configuration block for the allowed input types of the prompt attempt. See `allowed_input_types`.
    #[builder(into)]
    #[serde(rename = "allowedInputTypes")]
    pub r#allowed_input_types: Box<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAllowedInputTypes>,
    /// Configuration block for settings on audio and DTMF input. See `audio_and_dtmf_input_specification`.
    #[builder(into)]
    #[serde(rename = "audioAndDtmfInputSpecification")]
    pub r#audio_and_dtmf_input_specification: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecification>>,
    /// Which attempt to configure. Valid values are `Initial`, `Retry1`, `Retry2`, `Retry3`, `Retry4`, `Retry5`.
    #[builder(into)]
    #[serde(rename = "mapBlockKey")]
    pub r#map_block_key: String,
    /// Configuration block for the settings on text input. See `text_input_specification`.
    #[builder(into)]
    #[serde(rename = "textInputSpecification")]
    pub r#text_input_specification: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationTextInputSpecification>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecification {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("allow_interrupt".to_string(), self.r#allow_interrupt.to_pulumi_value().await);
            map.insert("allowed_input_types".to_string(), self.r#allowed_input_types.to_pulumi_value().await);
            map.insert("audio_and_dtmf_input_specification".to_string(), self.r#audio_and_dtmf_input_specification.to_pulumi_value().await);
            map.insert("map_block_key".to_string(), self.r#map_block_key.to_pulumi_value().await);
            map.insert("text_input_specification".to_string(), self.r#text_input_specification.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecification {
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
                    r#allow_interrupt: {
                        let field_value = match fields_map.get("allow_interrupt") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_interrupt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#allowed_input_types: {
                        let field_value = match fields_map.get("allowed_input_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_input_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAllowedInputTypes> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#audio_and_dtmf_input_specification: {
                        let field_value = match fields_map.get("audio_and_dtmf_input_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_and_dtmf_input_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#map_block_key: {
                        let field_value = match fields_map.get("map_block_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'map_block_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#text_input_specification: {
                        let field_value = match fields_map.get("text_input_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_input_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationTextInputSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
