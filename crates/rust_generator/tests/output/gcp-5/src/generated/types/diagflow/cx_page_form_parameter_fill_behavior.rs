#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxPageFormParameterFillBehavior {
    /// The fulfillment to provide the initial prompt that the agent can present to the user in order to fill the parameter.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "initialPromptFulfillment")]
    pub r#initial_prompt_fulfillment: Option<Box<super::super::types::diagflow::CxPageFormParameterFillBehaviorInitialPromptFulfillment>>,
    /// The handlers for parameter-level events, used to provide reprompt for the parameter or transition to a different page/flow. The supported events are:
    /// * sys.no-match-<N>, where N can be from 1 to 6
    /// * sys.no-match-default
    /// * sys.no-input-<N>, where N can be from 1 to 6
    /// * sys.no-input-default
    /// * sys.invalid-parameter
    /// [initialPromptFulfillment][initialPromptFulfillment] provides the first prompt for the parameter.
    /// If the user's response does not fill the parameter, a no-match/no-input event will be triggered, and the fulfillment associated with the sys.no-match-1/sys.no-input-1 handler (if defined) will be called to provide a prompt. The sys.no-match-2/sys.no-input-2 handler (if defined) will respond to the next no-match/no-input event, and so on.
    /// A sys.no-match-default or sys.no-input-default handler will be used to handle all following no-match/no-input events after all numbered no-match/no-input handlers for the parameter are consumed.
    /// A sys.invalid-parameter handler can be defined to handle the case where the parameter values have been invalidated by webhook. For example, if the user's response fill the parameter, however the parameter was invalidated by webhook, the fulfillment associated with the sys.invalid-parameter handler (if defined) will be called to provide a prompt.
    /// If the event handler for the corresponding event can't be found on the parameter, initialPromptFulfillment will be re-prompted.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "repromptEventHandlers")]
    pub r#reprompt_event_handlers: Option<Vec<super::super::types::diagflow::CxPageFormParameterFillBehaviorRepromptEventHandler>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxPageFormParameterFillBehavior {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "initial_prompt_fulfillment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#initial_prompt_fulfillment,
                )
                .await,
            );
            map.insert(
                "reprompt_event_handlers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reprompt_event_handlers,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxPageFormParameterFillBehavior {
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
                    r#initial_prompt_fulfillment: {
                        let field_value = match fields_map.get("initial_prompt_fulfillment") {
                            Some(value) => value,
                            None => bail!("Missing field 'initial_prompt_fulfillment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reprompt_event_handlers: {
                        let field_value = match fields_map.get("reprompt_event_handlers") {
                            Some(value) => value,
                            None => bail!("Missing field 'reprompt_event_handlers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
