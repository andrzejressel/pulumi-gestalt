#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentClosingSetting {
    /// Whether an intent's closing response is used. When this field is false, the closing response isn't sent to the user. If the active field isn't specified, the default is true.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: Option<bool>,
    /// Configuration block for response that Amazon Lex sends to the user when the intent is complete. See `closing_response`.
    #[builder(into)]
    #[serde(rename = "closingResponse")]
    pub r#closing_response: Option<Box<super::super::types::lex::V2ModelsIntentClosingSettingClosingResponse>>,
    /// Configuration block for list of conditional branches associated with the intent's closing response. These branches are executed when the `next_step` attribute is set to `EvalutateConditional`. See `conditional`.
    #[builder(into)]
    #[serde(rename = "conditional")]
    pub r#conditional: Option<Box<super::super::types::lex::V2ModelsIntentClosingSettingConditional>>,
    /// Next step that the bot executes after playing the intent's closing response. See `next_step`.
    #[builder(into)]
    #[serde(rename = "nextStep")]
    pub r#next_step: Option<Box<super::super::types::lex::V2ModelsIntentClosingSettingNextStep>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentClosingSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("active".to_string(), self.r#active.to_pulumi_value().await);
            map.insert("closing_response".to_string(), self.r#closing_response.to_pulumi_value().await);
            map.insert("conditional".to_string(), self.r#conditional.to_pulumi_value().await);
            map.insert("next_step".to_string(), self.r#next_step.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentClosingSetting {
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
                    r#active: {
                        let field_value = match fields_map.get("active") {
                            Some(value) => value,
                            None => bail!("Missing field 'active' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#closing_response: {
                        let field_value = match fields_map.get("closing_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'closing_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentClosingSettingClosingResponse>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#conditional: {
                        let field_value = match fields_map.get("conditional") {
                            Some(value) => value,
                            None => bail!("Missing field 'conditional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentClosingSettingConditional>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#next_step: {
                        let field_value = match fields_map.get("next_step") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_step' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentClosingSettingNextStep>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
