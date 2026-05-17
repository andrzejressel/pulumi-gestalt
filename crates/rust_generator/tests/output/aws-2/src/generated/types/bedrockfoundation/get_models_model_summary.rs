#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetModelsModelSummary {
    /// Customizations that the model supports.
    #[builder(into)]
    #[serde(rename = "customizationsSupporteds")]
    pub r#customizations_supporteds: Vec<String>,
    /// Inference types that the model supports.
    #[builder(into)]
    #[serde(rename = "inferenceTypesSupporteds")]
    pub r#inference_types_supporteds: Vec<String>,
    /// Input modalities that the model supports.
    #[builder(into)]
    #[serde(rename = "inputModalities")]
    pub r#input_modalities: Vec<String>,
    /// Model ARN.
    #[builder(into)]
    #[serde(rename = "modelArn")]
    pub r#model_arn: String,
    /// Model identifier.
    #[builder(into)]
    #[serde(rename = "modelId")]
    pub r#model_id: String,
    /// Model name.
    #[builder(into)]
    #[serde(rename = "modelName")]
    pub r#model_name: String,
    /// Output modalities that the model supports.
    #[builder(into)]
    #[serde(rename = "outputModalities")]
    pub r#output_modalities: Vec<String>,
    /// Model provider name.
    #[builder(into)]
    #[serde(rename = "providerName")]
    pub r#provider_name: String,
    /// Indicates whether the model supports streaming.
    #[builder(into)]
    #[serde(rename = "responseStreamingSupported")]
    pub r#response_streaming_supported: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetModelsModelSummary {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "customizations_supporteds",
                    &self.r#customizations_supporteds,
                ),
                to_pulumi_object_field(
                    "inference_types_supporteds",
                    &self.r#inference_types_supporteds,
                ),
                to_pulumi_object_field(
                    "input_modalities",
                    &self.r#input_modalities,
                ),
                to_pulumi_object_field(
                    "model_arn",
                    &self.r#model_arn,
                ),
                to_pulumi_object_field(
                    "model_id",
                    &self.r#model_id,
                ),
                to_pulumi_object_field(
                    "model_name",
                    &self.r#model_name,
                ),
                to_pulumi_object_field(
                    "output_modalities",
                    &self.r#output_modalities,
                ),
                to_pulumi_object_field(
                    "provider_name",
                    &self.r#provider_name,
                ),
                to_pulumi_object_field(
                    "response_streaming_supported",
                    &self.r#response_streaming_supported,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetModelsModelSummary {
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
                    r#customizations_supporteds: {
                        let field_value = match fields_map.get("customizations_supporteds") {
                            Some(value) => value,
                            None => bail!("Missing field 'customizations_supporteds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inference_types_supporteds: {
                        let field_value = match fields_map.get("inference_types_supporteds") {
                            Some(value) => value,
                            None => bail!("Missing field 'inference_types_supporteds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_modalities: {
                        let field_value = match fields_map.get("input_modalities") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_modalities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_arn: {
                        let field_value = match fields_map.get("model_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'model_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_id: {
                        let field_value = match fields_map.get("model_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'model_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_name: {
                        let field_value = match fields_map.get("model_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'model_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_modalities: {
                        let field_value = match fields_map.get("output_modalities") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_modalities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provider_name: {
                        let field_value = match fields_map.get("provider_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'provider_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_streaming_supported: {
                        let field_value = match fields_map.get("response_streaming_supported") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_streaming_supported' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
