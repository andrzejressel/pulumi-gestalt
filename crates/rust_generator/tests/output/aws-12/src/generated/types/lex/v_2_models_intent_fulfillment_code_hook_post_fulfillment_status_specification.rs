#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecification {
    /// Configuration block for conditional branches to evaluate after the dialog code hook throws an exception or returns with the State field of the Intent object set to Failed. See `failure_conditional`.
    #[builder(into)]
    #[serde(rename = "failureConditional")]
    pub r#failure_conditional: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationFailureConditional>>,
    /// Configuration block for the next step the bot runs after the dialog code hook throws an exception or returns with the State field of the Intent object set to Failed. See `failure_next_step`.
    #[builder(into)]
    #[serde(rename = "failureNextStep")]
    pub r#failure_next_step: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationFailureNextStep>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `failure_response`.
    #[builder(into)]
    #[serde(rename = "failureResponse")]
    pub r#failure_response: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationFailureResponse>>,
    /// Configuration block for conditional branches to evaluate after the dialog code hook finishes successfully. See `success_conditional`.
    #[builder(into)]
    #[serde(rename = "successConditional")]
    pub r#success_conditional: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationSuccessConditional>>,
    /// Configuration block for the next step the bot runs after the dialog code hook finishes successfully. See `success_next_step`.
    #[builder(into)]
    #[serde(rename = "successNextStep")]
    pub r#success_next_step: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationSuccessNextStep>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `success_response`.
    #[builder(into)]
    #[serde(rename = "successResponse")]
    pub r#success_response: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationSuccessResponse>>,
    /// Configuration block for conditional branches to evaluate if the code hook times out. See `timeout_conditional`.
    #[builder(into)]
    #[serde(rename = "timeoutConditional")]
    pub r#timeout_conditional: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationTimeoutConditional>>,
    /// Configuration block for the next step that the bot runs when the code hook times out. See `timeout_next_step`.
    #[builder(into)]
    #[serde(rename = "timeoutNextStep")]
    pub r#timeout_next_step: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationTimeoutNextStep>>,
    /// Configuration block for a list of message groups that Amazon Lex uses to respond the user input. See `timeout_response`.
    #[builder(into)]
    #[serde(rename = "timeoutResponse")]
    pub r#timeout_response: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationTimeoutResponse>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecification {
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
                "success_conditional".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#success_conditional,
                )
                .await,
            );
            map.insert(
                "success_next_step".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#success_next_step,
                )
                .await,
            );
            map.insert(
                "success_response".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#success_response,
                )
                .await,
            );
            map.insert(
                "timeout_conditional".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout_conditional,
                )
                .await,
            );
            map.insert(
                "timeout_next_step".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout_next_step,
                )
                .await,
            );
            map.insert(
                "timeout_response".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout_response,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecification {
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
                    r#success_conditional: {
                        let field_value = match fields_map.get("success_conditional") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_conditional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#success_next_step: {
                        let field_value = match fields_map.get("success_next_step") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_next_step' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#success_response: {
                        let field_value = match fields_map.get("success_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_conditional: {
                        let field_value = match fields_map.get("timeout_conditional") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_conditional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_next_step: {
                        let field_value = match fields_map.get("timeout_next_step") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_next_step' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_response: {
                        let field_value = match fields_map.get("timeout_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
