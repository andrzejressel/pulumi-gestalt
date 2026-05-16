#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotValueElicitationSettingPromptSpecification {
    #[builder(into)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Option<bool>,
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: i32,
    #[builder(into)]
    #[serde(rename = "messageGroups")]
    pub r#message_groups: Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationMessageGroup>>,
    #[builder(into)]
    #[serde(rename = "messageSelectionStrategy")]
    pub r#message_selection_strategy: Option<String>,
    #[builder(into)]
    #[serde(rename = "promptAttemptsSpecifications")]
    pub r#prompt_attempts_specifications: Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecification>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsSlotValueElicitationSettingPromptSpecification {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("allow_interrupt".to_string(), self.r#allow_interrupt.to_pulumi_value().await);
            map.insert("max_retries".to_string(), self.r#max_retries.to_pulumi_value().await);
            map.insert("message_groups".to_string(), self.r#message_groups.to_pulumi_value().await);
            map.insert("message_selection_strategy".to_string(), self.r#message_selection_strategy.to_pulumi_value().await);
            map.insert("prompt_attempts_specifications".to_string(), self.r#prompt_attempts_specifications.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsSlotValueElicitationSettingPromptSpecification {
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
                    r#allow_interrupt: {
                        let field_value = match fields_map.get("allow_interrupt") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_interrupt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_retries: {
                        let field_value = match fields_map.get("max_retries") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_retries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#message_groups: {
                        let field_value = match fields_map.get("message_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'message_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationMessageGroup>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#message_selection_strategy: {
                        let field_value = match fields_map.get("message_selection_strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'message_selection_strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prompt_attempts_specifications: {
                        let field_value = match fields_map.get("prompt_attempts_specifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'prompt_attempts_specifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
