#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSettingPromptSpecification {
    /// Whether the user can interrupt a speech prompt from the bot.
    #[builder(into)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Option<bool>,
    /// Maximum number of times the bot tries to elicit a response from the user using this prompt.
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: i32,
    /// Configuration block for messages that Amazon Lex can send to the user. Amazon Lex chooses the actual message to send at runtime. See `message_group`.
    #[builder(into)]
    #[serde(rename = "messageGroups")]
    pub r#message_groups: Option<Vec<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecificationMessageGroup>>,
    /// How a message is selected from a message group among retries. Valid values are `Random` and `Ordered`.
    #[builder(into)]
    #[serde(rename = "messageSelectionStrategy")]
    pub r#message_selection_strategy: Option<String>,
    /// Configuration block for advanced settings on each attempt of the prompt. See `prompt_attempts_specification`.
    #[builder(into)]
    #[serde(rename = "promptAttemptsSpecifications")]
    pub r#prompt_attempts_specifications: Option<Vec<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecification>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentConfirmationSettingPromptSpecification {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "allow_interrupt".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_interrupt,
                )
                .await,
            );
            map.insert(
                "max_retries".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_retries,
                )
                .await,
            );
            map.insert(
                "message_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#message_groups,
                )
                .await,
            );
            map.insert(
                "message_selection_strategy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#message_selection_strategy,
                )
                .await,
            );
            map.insert(
                "prompt_attempts_specifications".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prompt_attempts_specifications,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentConfirmationSettingPromptSpecification {
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
                    r#allow_interrupt: {
                        let field_value = match fields_map.get("allow_interrupt") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_interrupt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_retries: {
                        let field_value = match fields_map.get("max_retries") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_retries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#message_groups: {
                        let field_value = match fields_map.get("message_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'message_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#message_selection_strategy: {
                        let field_value = match fields_map.get("message_selection_strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'message_selection_strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prompt_attempts_specifications: {
                        let field_value = match fields_map.get("prompt_attempts_specifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'prompt_attempts_specifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
