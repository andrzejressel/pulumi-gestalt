#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotValueElicitationSettingWaitAndContinueSpecification {
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
    pub r#continue_responses: Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationContinueResponse>>,
    /// Response that Amazon Lex sends periodically to the user to indicate that the bot is still waiting for input from the user.
    /// See the `still_waiting_response` argument reference below.
    #[builder(into)]
    #[serde(rename = "stillWaitingResponses")]
    pub r#still_waiting_responses: Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponse>>,
    /// Response that Amazon Lex sends to indicate that the bot is waiting for the conversation to continue.
    /// See the `waiting_response` argument reference below.
    #[builder(into)]
    #[serde(rename = "waitingResponses")]
    pub r#waiting_responses: Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationWaitingResponse>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsSlotValueElicitationSettingWaitAndContinueSpecification {
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
                    "active",
                    &self.r#active,
                ),
                to_pulumi_object_field(
                    "continue_responses",
                    &self.r#continue_responses,
                ),
                to_pulumi_object_field(
                    "still_waiting_responses",
                    &self.r#still_waiting_responses,
                ),
                to_pulumi_object_field(
                    "waiting_responses",
                    &self.r#waiting_responses,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsSlotValueElicitationSettingWaitAndContinueSpecification {
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
