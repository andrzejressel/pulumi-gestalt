#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSetting {
    /// Whether the intent's confirmation is sent to the user. When this field is false, confirmation and declination responses aren't sent. If the active field isn't specified, the default is true.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: Option<bool>,
    /// Configuration block for the intent's confirmation step. The dialog code hook is triggered based on these invocation settings when the confirmation next step or declination next step or failure next step is `invoke_dialog_code_hook`.  See `code_hook`.
    #[builder(into)]
    #[serde(rename = "codeHook")]
    pub r#code_hook: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHook>>,
    /// Configuration block for conditional branches to evaluate after the intent is closed. See `confirmation_conditional`.
    #[builder(into)]
    #[serde(rename = "confirmationConditional")]
    pub r#confirmation_conditional: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingConfirmationConditional>>,
    /// Configuration block for the next step that the bot executes when the customer confirms the intent. See `confirmation_next_step`.
    #[builder(into)]
    #[serde(rename = "confirmationNextStep")]
    pub r#confirmation_next_step: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingConfirmationNextStep>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `confirmation_response`.
    #[builder(into)]
    #[serde(rename = "confirmationResponse")]
    pub r#confirmation_response: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingConfirmationResponse>>,
    /// Configuration block for conditional branches to evaluate after the intent is declined. See `declination_conditional`.
    #[builder(into)]
    #[serde(rename = "declinationConditional")]
    pub r#declination_conditional: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationConditional>>,
    /// Configuration block for the next step that the bot executes when the customer declines the intent. See `declination_next_step`.
    #[builder(into)]
    #[serde(rename = "declinationNextStep")]
    pub r#declination_next_step: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationNextStep>>,
    /// Configuration block for when the user answers "no" to the question defined in `prompt_specification`, Amazon Lex responds with this response to acknowledge that the intent was canceled. See `declination_response`.
    #[builder(into)]
    #[serde(rename = "declinationResponse")]
    pub r#declination_response: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationResponse>>,
    /// Configuration block for when the code hook is invoked during confirmation prompt retries. See `elicitation_code_hook`.
    #[builder(into)]
    #[serde(rename = "elicitationCodeHook")]
    pub r#elicitation_code_hook: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingElicitationCodeHook>>,
    /// Configuration block for conditional branches. Branches are evaluated in the order that they are entered in the list. The first branch with a condition that evaluates to true is executed. The last branch in the list is the default branch. The default branch should not have any condition expression. The default branch is executed if no other branch has a matching condition. See `failure_conditional`.
    #[builder(into)]
    #[serde(rename = "failureConditional")]
    pub r#failure_conditional: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditional>>,
    /// Configuration block for the next step to take in the conversation if the confirmation step fails. See `failure_next_step`.
    #[builder(into)]
    #[serde(rename = "failureNextStep")]
    pub r#failure_next_step: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureNextStep>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `failure_response`.
    #[builder(into)]
    #[serde(rename = "failureResponse")]
    pub r#failure_response: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureResponse>>,
    /// Configuration block for prompting the user to confirm the intent. This question should have a yes or no answer. Amazon Lex uses this prompt to ensure that the user acknowledges that the intent is ready for fulfillment. See `prompt_specification`.
    #[builder(into)]
    #[serde(rename = "promptSpecification")]
    pub r#prompt_specification: Box<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecification>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentConfirmationSetting {
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
                "active".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#active,
                )
                .await,
            );
            map.insert(
                "code_hook".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#code_hook,
                )
                .await,
            );
            map.insert(
                "confirmation_conditional".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#confirmation_conditional,
                )
                .await,
            );
            map.insert(
                "confirmation_next_step".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#confirmation_next_step,
                )
                .await,
            );
            map.insert(
                "confirmation_response".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#confirmation_response,
                )
                .await,
            );
            map.insert(
                "declination_conditional".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#declination_conditional,
                )
                .await,
            );
            map.insert(
                "declination_next_step".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#declination_next_step,
                )
                .await,
            );
            map.insert(
                "declination_response".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#declination_response,
                )
                .await,
            );
            map.insert(
                "elicitation_code_hook".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#elicitation_code_hook,
                )
                .await,
            );
            map.insert(
                "failure_conditional".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failure_conditional,
                )
                .await,
            );
            map.insert(
                "failure_next_step".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failure_next_step,
                )
                .await,
            );
            map.insert(
                "failure_response".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failure_response,
                )
                .await,
            );
            map.insert(
                "prompt_specification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prompt_specification,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentConfirmationSetting {
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
                    r#active: {
                        let field_value = match fields_map.get("active") {
                            Some(value) => value,
                            None => bail!("Missing field 'active' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#code_hook: {
                        let field_value = match fields_map.get("code_hook") {
                            Some(value) => value,
                            None => bail!("Missing field 'code_hook' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#confirmation_conditional: {
                        let field_value = match fields_map.get("confirmation_conditional") {
                            Some(value) => value,
                            None => bail!("Missing field 'confirmation_conditional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#confirmation_next_step: {
                        let field_value = match fields_map.get("confirmation_next_step") {
                            Some(value) => value,
                            None => bail!("Missing field 'confirmation_next_step' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#confirmation_response: {
                        let field_value = match fields_map.get("confirmation_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'confirmation_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#declination_conditional: {
                        let field_value = match fields_map.get("declination_conditional") {
                            Some(value) => value,
                            None => bail!("Missing field 'declination_conditional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#declination_next_step: {
                        let field_value = match fields_map.get("declination_next_step") {
                            Some(value) => value,
                            None => bail!("Missing field 'declination_next_step' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#declination_response: {
                        let field_value = match fields_map.get("declination_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'declination_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elicitation_code_hook: {
                        let field_value = match fields_map.get("elicitation_code_hook") {
                            Some(value) => value,
                            None => bail!("Missing field 'elicitation_code_hook' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failure_conditional: {
                        let field_value = match fields_map.get("failure_conditional") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_conditional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failure_next_step: {
                        let field_value = match fields_map.get("failure_next_step") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_next_step' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failure_response: {
                        let field_value = match fields_map.get("failure_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prompt_specification: {
                        let field_value = match fields_map.get("prompt_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'prompt_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
