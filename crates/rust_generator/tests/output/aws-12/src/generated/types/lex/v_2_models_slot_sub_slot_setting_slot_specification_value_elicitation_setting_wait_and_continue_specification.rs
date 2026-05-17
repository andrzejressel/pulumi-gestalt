#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecification {
    /// Specifies whether the bot will wait for a user to respond.
    /// When this field is `false`, wait and continue responses for a slot aren't used.
    /// If the active field isn't specified, the default is `true`.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: Option<bool>,
    /// Response that Amazon Lex sends to indicate that the bot is ready to continue the conversation.
    /// See the `continue_response` argument reference below.
    #[builder(into)]
    #[serde(rename = "continueResponses")]
    pub r#continue_responses: Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecificationContinueResponse>>,
    /// Response that Amazon Lex sends periodically to the user to indicate that the bot is still waiting for input from the user.
    /// See the `still_waiting_response` argument reference below.
    #[builder(into)]
    #[serde(rename = "stillWaitingResponses")]
    pub r#still_waiting_responses: Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponse>>,
    /// Response that Amazon Lex sends to indicate that the bot is waiting for the conversation to continue.
    /// See the `waiting_response` argument reference below.
    #[builder(into)]
    #[serde(rename = "waitingResponses")]
    pub r#waiting_responses: Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecificationWaitingResponse>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecification {
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
                "continue_responses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#continue_responses,
                )
                .await,
            );
            map.insert(
                "still_waiting_responses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#still_waiting_responses,
                )
                .await,
            );
            map.insert(
                "waiting_responses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#waiting_responses,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecification {
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
                    r#continue_responses: {
                        let field_value = match fields_map.get("continue_responses") {
                            Some(value) => value,
                            None => bail!("Missing field 'continue_responses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#still_waiting_responses: {
                        let field_value = match fields_map.get("still_waiting_responses") {
                            Some(value) => value,
                            None => bail!("Missing field 'still_waiting_responses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#waiting_responses: {
                        let field_value = match fields_map.get("waiting_responses") {
                            Some(value) => value,
                            None => bail!("Missing field 'waiting_responses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
