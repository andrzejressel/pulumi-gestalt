#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentAgentPromptOverrideConfigurationPromptConfigurationInferenceConfiguration {
    /// Maximum number of tokens to allow in the generated response.
    #[builder(into)]
    #[serde(rename = "maxLength")]
    pub r#max_length: i32,
    /// List of stop sequences. A stop sequence is a sequence of characters that causes the model to stop generating the response.
    #[builder(into)]
    #[serde(rename = "stopSequences")]
    pub r#stop_sequences: Vec<String>,
    /// Likelihood of the model selecting higher-probability options while generating a response. A lower value makes the model more likely to choose higher-probability options, while a higher value makes the model more likely to choose lower-probability options.
    #[builder(into)]
    #[serde(rename = "temperature")]
    pub r#temperature: f64,
    /// Number of top most-likely candidates, between 0 and 500, from which the model chooses the next token in the sequence.
    #[builder(into)]
    #[serde(rename = "topK")]
    pub r#top_k: i32,
    /// Top percentage of the probability distribution of next tokens, between 0 and 1 (denoting 0% and 100%), from which the model chooses the next token in the sequence.
    #[builder(into)]
    #[serde(rename = "topP")]
    pub r#top_p: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentAgentPromptOverrideConfigurationPromptConfigurationInferenceConfiguration {
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
                "max_length".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_length,
                )
                .await,
            );
            map.insert(
                "stop_sequences".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stop_sequences,
                )
                .await,
            );
            map.insert(
                "temperature".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#temperature,
                )
                .await,
            );
            map.insert(
                "top_k".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#top_k,
                )
                .await,
            );
            map.insert(
                "top_p".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#top_p,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentAgentPromptOverrideConfigurationPromptConfigurationInferenceConfiguration {
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
                    r#max_length: {
                        let field_value = match fields_map.get("max_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stop_sequences: {
                        let field_value = match fields_map.get("stop_sequences") {
                            Some(value) => value,
                            None => bail!("Missing field 'stop_sequences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#temperature: {
                        let field_value = match fields_map.get("temperature") {
                            Some(value) => value,
                            None => bail!("Missing field 'temperature' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#top_k: {
                        let field_value = match fields_map.get("top_k") {
                            Some(value) => value,
                            None => bail!("Missing field 'top_k' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#top_p: {
                        let field_value = match fields_map.get("top_p") {
                            Some(value) => value,
                            None => bail!("Missing field 'top_p' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
