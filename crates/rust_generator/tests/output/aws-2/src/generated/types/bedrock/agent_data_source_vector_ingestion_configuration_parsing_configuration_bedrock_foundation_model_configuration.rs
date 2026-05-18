#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentDataSourceVectorIngestionConfigurationParsingConfigurationBedrockFoundationModelConfiguration {
    /// The ARN of the model used to parse documents
    #[builder(into)]
    #[serde(rename = "modelArn")]
    pub r#model_arn: String,
    /// Instructions for interpreting the contents of the document. See `parsing_prompt` block for details.
    #[builder(into)]
    #[serde(rename = "parsingPrompt")]
    pub r#parsing_prompt: Option<Box<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationParsingConfigurationBedrockFoundationModelConfigurationParsingPrompt>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentDataSourceVectorIngestionConfigurationParsingConfigurationBedrockFoundationModelConfiguration {
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
                    "model_arn",
                    &self.r#model_arn,
                ),
                to_pulumi_object_field(
                    "parsing_prompt",
                    &self.r#parsing_prompt,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentDataSourceVectorIngestionConfigurationParsingConfigurationBedrockFoundationModelConfiguration {
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
                    r#model_arn: {
                        let field_value = match fields_map.get("model_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'model_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parsing_prompt: {
                        let field_value = match fields_map.get("parsing_prompt") {
                            Some(value) => value,
                            None => bail!("Missing field 'parsing_prompt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
