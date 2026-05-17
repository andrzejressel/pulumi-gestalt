#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxTestCaseTestCaseConversationTurnVirtualAgentOutput {
    /// The [Page](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows.pages#Page) on which the utterance was spoken.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "currentPage")]
    pub r#current_page: Option<Box<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnVirtualAgentOutputCurrentPage>>,
    /// The session parameters available to the bot at this point.
    #[builder(into)]
    #[serde(rename = "sessionParameters")]
    pub r#session_parameters: Option<String>,
    /// The text responses from the agent for the turn.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "textResponses")]
    pub r#text_responses: Option<Vec<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnVirtualAgentOutputTextResponse>>,
    /// The [Intent](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.intents#Intent) that triggered the response.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "triggeredIntent")]
    pub r#triggered_intent: Option<Box<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnVirtualAgentOutputTriggeredIntent>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxTestCaseTestCaseConversationTurnVirtualAgentOutput {
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
                "current_page".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#current_page,
                )
                .await,
            );
            map.insert(
                "session_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#session_parameters,
                )
                .await,
            );
            map.insert(
                "text_responses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#text_responses,
                )
                .await,
            );
            map.insert(
                "triggered_intent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#triggered_intent,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxTestCaseTestCaseConversationTurnVirtualAgentOutput {
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
                    r#current_page: {
                        let field_value = match fields_map.get("current_page") {
                            Some(value) => value,
                            None => bail!("Missing field 'current_page' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_parameters: {
                        let field_value = match fields_map.get("session_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text_responses: {
                        let field_value = match fields_map.get("text_responses") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_responses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#triggered_intent: {
                        let field_value = match fields_map.get("triggered_intent") {
                            Some(value) => value,
                            None => bail!("Missing field 'triggered_intent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
