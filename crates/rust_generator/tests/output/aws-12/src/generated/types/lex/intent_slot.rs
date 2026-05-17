#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntentSlot {
    /// A description of the bot. Must be less than or equal to 200 characters in length.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The name of the intent slot that you want to create. The name is case sensitive. Must be less than or equal to 100 characters in length.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Directs Lex the order in which to elicit this slot value from the user.
    /// For example, if the intent has two slots with priorities 1 and 2, AWS Lex first elicits a value for
    /// the slot with priority 1. If multiple slots share the same priority, the order in which Lex elicits
    /// values is arbitrary. Must be between 1 and 100.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
    /// The response card. Amazon Lex will substitute session attributes and
    /// slot values into the response card. For more information, see
    /// [Example: Using a Response Card](https://docs.aws.amazon.com/lex/latest/dg/ex-resp-card.html). Must be less than or equal to 50000 characters in length.
    #[builder(into)]
    #[serde(rename = "responseCard")]
    pub r#response_card: Option<String>,
    /// If you know a specific pattern with which users might respond to
    /// an Amazon Lex request for a slot value, you can provide those utterances to improve accuracy. This
    /// is optional. In most cases, Amazon Lex is capable of understanding user utterances. Must have between 1 and 10 items in the list, and each item must be less than or equal to 200 characters in length.
    #[builder(into)]
    #[serde(rename = "sampleUtterances")]
    pub r#sample_utterances: Option<Vec<String>>,
    /// Specifies whether the slot is required or optional.
    #[builder(into)]
    #[serde(rename = "slotConstraint")]
    pub r#slot_constraint: String,
    /// The type of the slot, either a custom slot type that you defined or one of
    /// the built-in slot types. Must be less than or equal to 100 characters in length.
    #[builder(into)]
    #[serde(rename = "slotType")]
    pub r#slot_type: String,
    /// The version of the slot type. Must be less than or equal to 64 characters in length.
    #[builder(into)]
    #[serde(rename = "slotTypeVersion")]
    pub r#slot_type_version: Option<String>,
    /// The prompt that Amazon Lex uses to elicit the slot value
    /// from the user. Attributes are documented under prompt.
    #[builder(into)]
    #[serde(rename = "valueElicitationPrompt")]
    pub r#value_elicitation_prompt: Option<Box<super::super::types::lex::IntentSlotValueElicitationPrompt>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IntentSlot {
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
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "priority".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#priority,
                )
                .await,
            );
            map.insert(
                "response_card".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#response_card,
                )
                .await,
            );
            map.insert(
                "sample_utterances".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sample_utterances,
                )
                .await,
            );
            map.insert(
                "slot_constraint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#slot_constraint,
                )
                .await,
            );
            map.insert(
                "slot_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#slot_type,
                )
                .await,
            );
            map.insert(
                "slot_type_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#slot_type_version,
                )
                .await,
            );
            map.insert(
                "value_elicitation_prompt".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#value_elicitation_prompt,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IntentSlot {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_card: {
                        let field_value = match fields_map.get("response_card") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_card' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sample_utterances: {
                        let field_value = match fields_map.get("sample_utterances") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_utterances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slot_constraint: {
                        let field_value = match fields_map.get("slot_constraint") {
                            Some(value) => value,
                            None => bail!("Missing field 'slot_constraint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slot_type: {
                        let field_value = match fields_map.get("slot_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'slot_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slot_type_version: {
                        let field_value = match fields_map.get("slot_type_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'slot_type_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value_elicitation_prompt: {
                        let field_value = match fields_map.get("value_elicitation_prompt") {
                            Some(value) => value,
                            None => bail!("Missing field 'value_elicitation_prompt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
