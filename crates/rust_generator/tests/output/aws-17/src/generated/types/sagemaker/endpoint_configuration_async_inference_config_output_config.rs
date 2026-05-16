#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointConfigurationAsyncInferenceConfigOutputConfig {
    /// The Amazon Web Services Key Management Service (Amazon Web Services KMS) key that Amazon SageMaker uses to encrypt the asynchronous inference output in Amazon S3.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Specifies the configuration for notifications of inference results for asynchronous inference.
    #[builder(into)]
    #[serde(rename = "notificationConfig")]
    pub r#notification_config: Option<Box<super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfigOutputConfigNotificationConfig>>,
    /// The Amazon S3 location to upload failure inference responses to.
    #[builder(into)]
    #[serde(rename = "s3FailurePath")]
    pub r#s_3_failure_path: Option<String>,
    /// The Amazon S3 location to upload inference responses to.
    #[builder(into)]
    #[serde(rename = "s3OutputPath")]
    pub r#s_3_output_path: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointConfigurationAsyncInferenceConfigOutputConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("kms_key_id".to_string(), self.r#kms_key_id.to_pulumi_value().await);
            map.insert("notification_config".to_string(), self.r#notification_config.to_pulumi_value().await);
            map.insert("s_3_failure_path".to_string(), self.r#s_3_failure_path.to_pulumi_value().await);
            map.insert("s_3_output_path".to_string(), self.r#s_3_output_path.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointConfigurationAsyncInferenceConfigOutputConfig {
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
                    r#kms_key_id: {
                        let field_value = match fields_map.get("kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#notification_config: {
                        let field_value = match fields_map.get("notification_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfigOutputConfigNotificationConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_failure_path: {
                        let field_value = match fields_map.get("s_3_failure_path") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_failure_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_output_path: {
                        let field_value = match fields_map.get("s_3_output_path") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_output_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
