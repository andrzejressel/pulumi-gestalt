#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InvocationLoggingConfigurationLoggingConfig {
    /// CloudWatch logging configuration.
    #[builder(into)]
    #[serde(rename = "cloudwatchConfig")]
    pub r#cloudwatch_config: Option<Box<super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfigCloudwatchConfig>>,
    /// Set to include embeddings data in the log delivery.
    #[builder(into)]
    #[serde(rename = "embeddingDataDeliveryEnabled")]
    pub r#embedding_data_delivery_enabled: bool,
    /// Set to include image data in the log delivery.
    #[builder(into)]
    #[serde(rename = "imageDataDeliveryEnabled")]
    pub r#image_data_delivery_enabled: bool,
    /// S3 configuration for storing log data.
    #[builder(into)]
    #[serde(rename = "s3Config")]
    pub r#s_3_config: Option<Box<super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfigS3Config>>,
    /// Set to include text data in the log delivery.
    #[builder(into)]
    #[serde(rename = "textDataDeliveryEnabled")]
    pub r#text_data_delivery_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InvocationLoggingConfigurationLoggingConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("cloudwatch_config".to_string(), self.r#cloudwatch_config.to_pulumi_value().await);
            map.insert("embedding_data_delivery_enabled".to_string(), self.r#embedding_data_delivery_enabled.to_pulumi_value().await);
            map.insert("image_data_delivery_enabled".to_string(), self.r#image_data_delivery_enabled.to_pulumi_value().await);
            map.insert("s_3_config".to_string(), self.r#s_3_config.to_pulumi_value().await);
            map.insert("text_data_delivery_enabled".to_string(), self.r#text_data_delivery_enabled.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InvocationLoggingConfigurationLoggingConfig {
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
                    r#cloudwatch_config: {
                        let field_value = match fields_map.get("cloudwatch_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfigCloudwatchConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#embedding_data_delivery_enabled: {
                        let field_value = match fields_map.get("embedding_data_delivery_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'embedding_data_delivery_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#image_data_delivery_enabled: {
                        let field_value = match fields_map.get("image_data_delivery_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_data_delivery_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_config: {
                        let field_value = match fields_map.get("s_3_config") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfigS3Config>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#text_data_delivery_enabled: {
                        let field_value = match fields_map.get("text_data_delivery_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_data_delivery_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
