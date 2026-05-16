#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InvocationLoggingConfigurationLoggingConfigCloudwatchConfig {
    /// S3 configuration for delivering a large amount of data.
    #[builder(into)]
    #[serde(rename = "largeDataDeliveryS3Config")]
    pub r#large_data_delivery_s_3_config: Option<Box<super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfigCloudwatchConfigLargeDataDeliveryS3Config>>,
    /// Log group name.
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: Option<String>,
    /// The role ARN.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InvocationLoggingConfigurationLoggingConfigCloudwatchConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("large_data_delivery_s_3_config".to_string(), self.r#large_data_delivery_s_3_config.to_pulumi_value().await);
            map.insert("log_group_name".to_string(), self.r#log_group_name.to_pulumi_value().await);
            map.insert("role_arn".to_string(), self.r#role_arn.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InvocationLoggingConfigurationLoggingConfigCloudwatchConfig {
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
                    r#large_data_delivery_s_3_config: {
                        let field_value = match fields_map.get("large_data_delivery_s_3_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'large_data_delivery_s_3_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfigCloudwatchConfigLargeDataDeliveryS3Config>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#log_group_name: {
                        let field_value = match fields_map.get("log_group_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_group_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
