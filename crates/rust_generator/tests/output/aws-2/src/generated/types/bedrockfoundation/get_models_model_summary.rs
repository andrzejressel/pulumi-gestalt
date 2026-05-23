#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetModelsModelSummary {
    /// Customizations that the model supports.
    #[builder(into)]
    pub r#customizations_supporteds: Vec<String>,
    /// Inference types that the model supports.
    #[builder(into)]
    pub r#inference_types_supporteds: Vec<String>,
    /// Input modalities that the model supports.
    #[builder(into)]
    pub r#input_modalities: Vec<String>,
    /// Model ARN.
    #[builder(into)]
    pub r#model_arn: String,
    /// Model identifier.
    #[builder(into)]
    pub r#model_id: String,
    /// Model name.
    #[builder(into)]
    pub r#model_name: String,
    /// Output modalities that the model supports.
    #[builder(into)]
    pub r#output_modalities: Vec<String>,
    /// Model provider name.
    #[builder(into)]
    pub r#provider_name: String,
    /// Indicates whether the model supports streaming.
    #[builder(into)]
    pub r#response_streaming_supported: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetModelsModelSummary {
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
                    "customizationsSupporteds",
                    &self.r#customizations_supporteds,
                ),
                to_pulumi_object_field(
                    "inferenceTypesSupporteds",
                    &self.r#inference_types_supporteds,
                ),
                to_pulumi_object_field(
                    "inputModalities",
                    &self.r#input_modalities,
                ),
                to_pulumi_object_field(
                    "modelArn",
                    &self.r#model_arn,
                ),
                to_pulumi_object_field(
                    "modelId",
                    &self.r#model_id,
                ),
                to_pulumi_object_field(
                    "modelName",
                    &self.r#model_name,
                ),
                to_pulumi_object_field(
                    "outputModalities",
                    &self.r#output_modalities,
                ),
                to_pulumi_object_field(
                    "providerName",
                    &self.r#provider_name,
                ),
                to_pulumi_object_field(
                    "responseStreamingSupported",
                    &self.r#response_streaming_supported,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("customizationsSupporteds") {
                            Some(value) => value,
                            None => bail!("Missing field 'customizationsSupporteds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inference_types_supporteds: {
                        let field_value = match fields_map.get("inferenceTypesSupporteds") {
                            Some(value) => value,
                            None => bail!("Missing field 'inferenceTypesSupporteds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_modalities: {
                        let field_value = match fields_map.get("inputModalities") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputModalities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_arn: {
                        let field_value = match fields_map.get("modelArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'modelArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_id: {
                        let field_value = match fields_map.get("modelId") {
                            Some(value) => value,
                            None => bail!("Missing field 'modelId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_name: {
                        let field_value = match fields_map.get("modelName") {
                            Some(value) => value,
                            None => bail!("Missing field 'modelName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_modalities: {
                        let field_value = match fields_map.get("outputModalities") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputModalities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provider_name: {
                        let field_value = match fields_map.get("providerName") {
                            Some(value) => value,
                            None => bail!("Missing field 'providerName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_streaming_supported: {
                        let field_value = match fields_map.get("responseStreamingSupported") {
                            Some(value) => value,
                            None => bail!("Missing field 'responseStreamingSupported' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
