#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxTestCaseLastTestResultConversationTurnUserInput {
    /// Whether sentiment analysis is enabled.
    #[builder(into)]
    #[serde(rename = "enableSentimentAnalysis")]
    pub r#enable_sentiment_analysis: Option<bool>,
    /// Parameters that need to be injected into the conversation during intent detection.
    #[builder(into)]
    #[serde(rename = "injectedParameters")]
    pub r#injected_parameters: Option<String>,
    /// User input. Supports text input, event input, dtmf input in the test case.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "input")]
    pub r#input: Option<Box<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnUserInputInput>>,
    /// If webhooks should be allowed to trigger in response to the user utterance. Often if parameters are injected, webhooks should not be enabled.
    #[builder(into)]
    #[serde(rename = "isWebhookEnabled")]
    pub r#is_webhook_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxTestCaseLastTestResultConversationTurnUserInput {
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
                "enable_sentiment_analysis".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_sentiment_analysis,
                )
                .await,
            );
            map.insert(
                "injected_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#injected_parameters,
                )
                .await,
            );
            map.insert(
                "input".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input,
                )
                .await,
            );
            map.insert(
                "is_webhook_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_webhook_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxTestCaseLastTestResultConversationTurnUserInput {
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
                    r#enable_sentiment_analysis: {
                        let field_value = match fields_map.get("enable_sentiment_analysis") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_sentiment_analysis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#injected_parameters: {
                        let field_value = match fields_map.get("injected_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'injected_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input: {
                        let field_value = match fields_map.get("input") {
                            Some(value) => value,
                            None => bail!("Missing field 'input' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_webhook_enabled: {
                        let field_value = match fields_map.get("is_webhook_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_webhook_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
