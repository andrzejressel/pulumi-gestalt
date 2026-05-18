#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointConfigurationDataCaptureConfig {
    /// The content type headers to capture. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "captureContentTypeHeader")]
    pub r#capture_content_type_header: Option<Box<super::super::types::sagemaker::EndpointConfigurationDataCaptureConfigCaptureContentTypeHeader>>,
    /// Specifies what data to capture. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "captureOptions")]
    pub r#capture_options: Vec<super::super::types::sagemaker::EndpointConfigurationDataCaptureConfigCaptureOption>,
    /// The URL for S3 location where the captured data is stored.
    #[builder(into)]
    #[serde(rename = "destinationS3Uri")]
    pub r#destination_s_3_uri: String,
    /// Flag to enable data capture. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enableCapture")]
    pub r#enable_capture: Option<bool>,
    /// Portion of data to capture. Should be between 0 and 100.
    #[builder(into)]
    #[serde(rename = "initialSamplingPercentage")]
    pub r#initial_sampling_percentage: i32,
    /// Amazon Resource Name (ARN) of a AWS Key Management Service key that Amazon SageMaker uses to encrypt the captured data on Amazon S3.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointConfigurationDataCaptureConfig {
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
                    "capture_content_type_header",
                    &self.r#capture_content_type_header,
                ),
                to_pulumi_object_field(
                    "capture_options",
                    &self.r#capture_options,
                ),
                to_pulumi_object_field(
                    "destination_s_3_uri",
                    &self.r#destination_s_3_uri,
                ),
                to_pulumi_object_field(
                    "enable_capture",
                    &self.r#enable_capture,
                ),
                to_pulumi_object_field(
                    "initial_sampling_percentage",
                    &self.r#initial_sampling_percentage,
                ),
                to_pulumi_object_field(
                    "kms_key_id",
                    &self.r#kms_key_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointConfigurationDataCaptureConfig {
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
                    r#capture_content_type_header: {
                        let field_value = match fields_map.get("capture_content_type_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'capture_content_type_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#capture_options: {
                        let field_value = match fields_map.get("capture_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'capture_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_s_3_uri: {
                        let field_value = match fields_map.get("destination_s_3_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_s_3_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_capture: {
                        let field_value = match fields_map.get("enable_capture") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_capture' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initial_sampling_percentage: {
                        let field_value = match fields_map.get("initial_sampling_percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'initial_sampling_percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_id: {
                        let field_value = match fields_map.get("kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
