#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecification {
    /// Configuration block for conditional branches to evaluate after the dialog code hook throws an exception or returns with the State field of the Intent object set to Failed.
    #[builder(into)]
    #[serde(rename = "failureConditional")]
    pub r#failure_conditional: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationFailureConditional>>,
    /// Configuration block for the next step the bot runs after the dialog code hook throws an exception or returns with the State field of the Intent object set to Failed . See `failure_next_step`.
    #[builder(into)]
    #[serde(rename = "failureNextStep")]
    pub r#failure_next_step: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationFailureNextStep>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `failure_response`.
    #[builder(into)]
    #[serde(rename = "failureResponse")]
    pub r#failure_response: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationFailureResponse>>,
    /// Configuration block for conditional branches to evaluate after the dialog code hook finishes successfully. See `success_conditional`.
    #[builder(into)]
    #[serde(rename = "successConditional")]
    pub r#success_conditional: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationSuccessConditional>>,
    /// Configuration block for the next step the bot runs after the dialog code hook finishes successfully. See `success_next_step`.
    #[builder(into)]
    #[serde(rename = "successNextStep")]
    pub r#success_next_step: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationSuccessNextStep>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `success_response`.
    #[builder(into)]
    #[serde(rename = "successResponse")]
    pub r#success_response: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationSuccessResponse>>,
    /// Configuration block for conditional branches to evaluate if the code hook times out. See `timeout_conditional`.
    #[builder(into)]
    #[serde(rename = "timeoutConditional")]
    pub r#timeout_conditional: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationTimeoutConditional>>,
    /// Configuration block for the next step that the bot runs when the code hook times out. See `timeout_next_step`.
    #[builder(into)]
    #[serde(rename = "timeoutNextStep")]
    pub r#timeout_next_step: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationTimeoutNextStep>>,
    /// Configuration block for a list of message groups that Amazon Lex uses to respond the user input. See `timeout_response`.
    #[builder(into)]
    #[serde(rename = "timeoutResponse")]
    pub r#timeout_response: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationTimeoutResponse>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecification {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("failure_conditional".to_string(), self.r#failure_conditional.to_pulumi_value().await);
            map.insert("failure_next_step".to_string(), self.r#failure_next_step.to_pulumi_value().await);
            map.insert("failure_response".to_string(), self.r#failure_response.to_pulumi_value().await);
            map.insert("success_conditional".to_string(), self.r#success_conditional.to_pulumi_value().await);
            map.insert("success_next_step".to_string(), self.r#success_next_step.to_pulumi_value().await);
            map.insert("success_response".to_string(), self.r#success_response.to_pulumi_value().await);
            map.insert("timeout_conditional".to_string(), self.r#timeout_conditional.to_pulumi_value().await);
            map.insert("timeout_next_step".to_string(), self.r#timeout_next_step.to_pulumi_value().await);
            map.insert("timeout_response".to_string(), self.r#timeout_response.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecification {
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
                    r#failure_conditional: {
                        let field_value = match fields_map.get("failure_conditional") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_conditional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationFailureConditional>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#failure_next_step: {
                        let field_value = match fields_map.get("failure_next_step") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_next_step' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationFailureNextStep>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#failure_response: {
                        let field_value = match fields_map.get("failure_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationFailureResponse>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#success_conditional: {
                        let field_value = match fields_map.get("success_conditional") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_conditional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationSuccessConditional>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#success_next_step: {
                        let field_value = match fields_map.get("success_next_step") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_next_step' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationSuccessNextStep>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#success_response: {
                        let field_value = match fields_map.get("success_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationSuccessResponse>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timeout_conditional: {
                        let field_value = match fields_map.get("timeout_conditional") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_conditional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationTimeoutConditional>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timeout_next_step: {
                        let field_value = match fields_map.get("timeout_next_step") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_next_step' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationTimeoutNextStep>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timeout_response: {
                        let field_value = match fields_map.get("timeout_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationTimeoutResponse>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
