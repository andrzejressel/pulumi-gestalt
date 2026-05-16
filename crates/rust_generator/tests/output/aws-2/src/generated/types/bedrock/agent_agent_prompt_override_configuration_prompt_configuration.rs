#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentAgentPromptOverrideConfigurationPromptConfiguration {
    /// prompt template with which to replace the default prompt template. You can use placeholder variables in the base prompt template to customize the prompt. For more information, see [Prompt template placeholder variables](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-placeholders.html).
    #[builder(into)]
    #[serde(rename = "basePromptTemplate")]
    pub r#base_prompt_template: String,
    /// Inference parameters to use when the agent invokes a foundation model in the part of the agent sequence defined by the `prompt_type`. For more information, see [Inference parameters for foundation models](https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html). See `inference_configuration` Block for details.
    #[builder(into)]
    #[serde(rename = "inferenceConfigurations")]
    pub r#inference_configurations: Vec<super::super::types::bedrock::AgentAgentPromptOverrideConfigurationPromptConfigurationInferenceConfiguration>,
    /// Whether to override the default parser Lambda function when parsing the raw foundation model output in the part of the agent sequence defined by the `prompt_type`. If you set the argument as `OVERRIDDEN`, the `override_lambda` argument in the `prompt_override_configuration` block must be specified with the ARN of a Lambda function. Valid values: `DEFAULT`, `OVERRIDDEN`.
    #[builder(into)]
    #[serde(rename = "parserMode")]
    pub r#parser_mode: String,
    /// Whether to override the default prompt template for this `prompt_type`. Set this argument to `OVERRIDDEN` to use the prompt that you provide in the `base_prompt_template`. If you leave it as `DEFAULT`, the agent uses a default prompt template. Valid values: `DEFAULT`, `OVERRIDDEN`.
    #[builder(into)]
    #[serde(rename = "promptCreationMode")]
    pub r#prompt_creation_mode: String,
    /// Whether to allow the agent to carry out the step specified in the `prompt_type`. If you set this argument to `DISABLED`, the agent skips that step. Valid Values: `ENABLED`, `DISABLED`.
    #[builder(into)]
    #[serde(rename = "promptState")]
    pub r#prompt_state: String,
    /// Step in the agent sequence that this prompt configuration applies to. Valid values: `PRE_PROCESSING`, `ORCHESTRATION`, `POST_PROCESSING`, `KNOWLEDGE_BASE_RESPONSE_GENERATION`.
    #[builder(into)]
    #[serde(rename = "promptType")]
    pub r#prompt_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentAgentPromptOverrideConfigurationPromptConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("base_prompt_template".to_string(), self.r#base_prompt_template.to_pulumi_value().await);
            map.insert("inference_configurations".to_string(), self.r#inference_configurations.to_pulumi_value().await);
            map.insert("parser_mode".to_string(), self.r#parser_mode.to_pulumi_value().await);
            map.insert("prompt_creation_mode".to_string(), self.r#prompt_creation_mode.to_pulumi_value().await);
            map.insert("prompt_state".to_string(), self.r#prompt_state.to_pulumi_value().await);
            map.insert("prompt_type".to_string(), self.r#prompt_type.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentAgentPromptOverrideConfigurationPromptConfiguration {
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
                    r#base_prompt_template: {
                        let field_value = match fields_map.get("base_prompt_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_prompt_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#inference_configurations: {
                        let field_value = match fields_map.get("inference_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'inference_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::bedrock::AgentAgentPromptOverrideConfigurationPromptConfigurationInferenceConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#parser_mode: {
                        let field_value = match fields_map.get("parser_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'parser_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prompt_creation_mode: {
                        let field_value = match fields_map.get("prompt_creation_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'prompt_creation_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prompt_state: {
                        let field_value = match fields_map.get("prompt_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'prompt_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prompt_type: {
                        let field_value = match fields_map.get("prompt_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'prompt_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
