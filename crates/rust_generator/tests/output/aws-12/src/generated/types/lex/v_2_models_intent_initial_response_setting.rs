#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentInitialResponseSetting {
    /// Configuration block for the dialog code hook that is called by Amazon Lex at a step of the conversation. See `code_hook`.
    #[builder(into)]
    #[serde(rename = "codeHook")]
    pub r#code_hook: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHook>>,
    /// Configuration block for conditional branches. Branches are evaluated in the order that they are entered in the list. The first branch with a condition that evaluates to true is executed. The last branch in the list is the default branch. The default branch should not have any condition expression. The default branch is executed if no other branch has a matching condition. See `conditional`.
    #[builder(into)]
    #[serde(rename = "conditional")]
    pub r#conditional: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingConditional>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `initial_response`.
    #[builder(into)]
    #[serde(rename = "initialResponse")]
    pub r#initial_response: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingInitialResponse>>,
    /// Configuration block for the next step in the conversation. See `next_step`.
    #[builder(into)]
    #[serde(rename = "nextStep")]
    pub r#next_step: Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingNextStep>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentInitialResponseSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("code_hook".to_string(), self.r#code_hook.to_pulumi_value().await);
            map.insert("conditional".to_string(), self.r#conditional.to_pulumi_value().await);
            map.insert("initial_response".to_string(), self.r#initial_response.to_pulumi_value().await);
            map.insert("next_step".to_string(), self.r#next_step.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentInitialResponseSetting {
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
                    r#code_hook: {
                        let field_value = match fields_map.get("code_hook") {
                            Some(value) => value,
                            None => bail!("Missing field 'code_hook' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHook>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#conditional: {
                        let field_value = match fields_map.get("conditional") {
                            Some(value) => value,
                            None => bail!("Missing field 'conditional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingConditional>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#initial_response: {
                        let field_value = match fields_map.get("initial_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'initial_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingInitialResponse>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#next_step: {
                        let field_value = match fields_map.get("next_step") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_step' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingNextStep>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
