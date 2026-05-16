#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntentSlotValueElicitationPrompt {
    /// The number of times to prompt the user for information. Must be a number between 1 and 5 (inclusive).
    #[builder(into)]
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: i32,
    #[builder(into)]
    #[serde(rename = "messages")]
    pub r#messages: Vec<super::super::types::lex::IntentSlotValueElicitationPromptMessage>,
    #[builder(into)]
    #[serde(rename = "responseCard")]
    pub r#response_card: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IntentSlotValueElicitationPrompt {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("max_attempts".to_string(), self.r#max_attempts.to_pulumi_value().await);
            map.insert("messages".to_string(), self.r#messages.to_pulumi_value().await);
            map.insert("response_card".to_string(), self.r#response_card.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IntentSlotValueElicitationPrompt {
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
                    r#max_attempts: {
                        let field_value = match fields_map.get("max_attempts") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_attempts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#messages: {
                        let field_value = match fields_map.get("messages") {
                            Some(value) => value,
                            None => bail!("Missing field 'messages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::lex::IntentSlotValueElicitationPromptMessage> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#response_card: {
                        let field_value = match fields_map.get("response_card") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_card' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
