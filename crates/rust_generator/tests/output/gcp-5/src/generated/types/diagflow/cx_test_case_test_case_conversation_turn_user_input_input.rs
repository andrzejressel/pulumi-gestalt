#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxTestCaseTestCaseConversationTurnUserInputInput {
    /// The DTMF event to be handled.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dtmf")]
    pub r#dtmf: Option<Box<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnUserInputInputDtmf>>,
    /// The event to be triggered.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "event")]
    pub r#event: Option<Box<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnUserInputInputEvent>>,
    /// The language of the input. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes.
    /// Note that queries in the same session do not necessarily need to specify the same language.
    #[builder(into)]
    #[serde(rename = "languageCode")]
    pub r#language_code: Option<String>,
    /// The natural language text to be processed.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "text")]
    pub r#text: Option<Box<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnUserInputInputText>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxTestCaseTestCaseConversationTurnUserInputInput {
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
                "dtmf".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dtmf,
                )
                .await,
            );
            map.insert(
                "event".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#event,
                )
                .await,
            );
            map.insert(
                "language_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#language_code,
                )
                .await,
            );
            map.insert(
                "text".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#text,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxTestCaseTestCaseConversationTurnUserInputInput {
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
                    r#dtmf: {
                        let field_value = match fields_map.get("dtmf") {
                            Some(value) => value,
                            None => bail!("Missing field 'dtmf' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event: {
                        let field_value = match fields_map.get("event") {
                            Some(value) => value,
                            None => bail!("Missing field 'event' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#language_code: {
                        let field_value = match fields_map.get("language_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'language_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text: {
                        let field_value = match fields_map.get("text") {
                            Some(value) => value,
                            None => bail!("Missing field 'text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
