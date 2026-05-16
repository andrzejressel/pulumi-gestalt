#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSetting {
    /// List of default values for a slot.
    /// See the `default_value_specification` argument reference below.
    #[builder(into)]
    #[serde(rename = "defaultValueSpecifications")]
    pub r#default_value_specifications: Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingDefaultValueSpecification>>,
    /// Prompt that Amazon Lex uses to elicit the slot value from the user.
    /// See the `aws.lex.V2modelsIntent` resource for details on the `prompt_specification` argument reference - they are identical.
    #[builder(into)]
    #[serde(rename = "promptSpecification")]
    pub r#prompt_specification: Box<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingPromptSpecification>,
    #[builder(into)]
    #[serde(rename = "sampleUtterances")]
    pub r#sample_utterances: Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingSampleUtterance>>,
    /// Specifies the prompts that Amazon Lex uses while a bot is waiting for customer input.
    /// See the `wait_and_continue_specification` argument reference below.
    #[builder(into)]
    #[serde(rename = "waitAndContinueSpecifications")]
    pub r#wait_and_continue_specifications: Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecification>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("default_value_specifications".to_string(), self.r#default_value_specifications.to_pulumi_value().await);
            map.insert("prompt_specification".to_string(), self.r#prompt_specification.to_pulumi_value().await);
            map.insert("sample_utterances".to_string(), self.r#sample_utterances.to_pulumi_value().await);
            map.insert("wait_and_continue_specifications".to_string(), self.r#wait_and_continue_specifications.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSetting {
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
                    r#default_value_specifications: {
                        let field_value = match fields_map.get("default_value_specifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_value_specifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingDefaultValueSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prompt_specification: {
                        let field_value = match fields_map.get("prompt_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'prompt_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingPromptSpecification> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sample_utterances: {
                        let field_value = match fields_map.get("sample_utterances") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_utterances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingSampleUtterance>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#wait_and_continue_specifications: {
                        let field_value = match fields_map.get("wait_and_continue_specifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'wait_and_continue_specifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
