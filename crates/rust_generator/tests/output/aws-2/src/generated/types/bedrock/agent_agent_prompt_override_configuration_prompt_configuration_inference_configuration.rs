#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentAgentPromptOverrideConfigurationPromptConfigurationInferenceConfiguration {
    /// Maximum number of tokens to allow in the generated response.
    #[builder(into)]
    pub r#max_length: i32,
    /// List of stop sequences. A stop sequence is a sequence of characters that causes the model to stop generating the response.
    #[builder(into)]
    pub r#stop_sequences: Vec<String>,
    /// Likelihood of the model selecting higher-probability options while generating a response. A lower value makes the model more likely to choose higher-probability options, while a higher value makes the model more likely to choose lower-probability options.
    #[builder(into)]
    pub r#temperature: f64,
    /// Number of top most-likely candidates, between 0 and 500, from which the model chooses the next token in the sequence.
    #[builder(into)]
    pub r#top_k: i32,
    /// Top percentage of the probability distribution of next tokens, between 0 and 1 (denoting 0% and 100%), from which the model chooses the next token in the sequence.
    #[builder(into)]
    pub r#top_p: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentAgentPromptOverrideConfigurationPromptConfigurationInferenceConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "maxLength",
                    &self.r#max_length,
                ),
                to_pulumi_object_field(
                    "stopSequences",
                    &self.r#stop_sequences,
                ),
                to_pulumi_object_field(
                    "temperature",
                    &self.r#temperature,
                ),
                to_pulumi_object_field(
                    "topK",
                    &self.r#top_k,
                ),
                to_pulumi_object_field(
                    "topP",
                    &self.r#top_p,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("maxLength") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxLength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stop_sequences: {
                        let field_value = match fields_map.get("stopSequences") {
                            Some(value) => value,
                            None => bail!("Missing field 'stopSequences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("topK") {
                            Some(value) => value,
                            None => bail!("Missing field 'topK' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#top_p: {
                        let field_value = match fields_map.get("topP") {
                            Some(value) => value,
                            None => bail!("Missing field 'topP' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
