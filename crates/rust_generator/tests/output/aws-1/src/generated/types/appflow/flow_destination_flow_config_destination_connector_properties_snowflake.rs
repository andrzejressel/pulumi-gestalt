#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflake {
    #[builder(into)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Option<String>,
    #[builder(into)]
    #[serde(rename = "errorHandlingConfig")]
    pub r#error_handling_config: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflakeErrorHandlingConfig>>,
    #[builder(into)]
    #[serde(rename = "intermediateBucketName")]
    pub r#intermediate_bucket_name: String,
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflake {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("bucket_prefix".to_string(), self.r#bucket_prefix.to_pulumi_value().await);
            map.insert("error_handling_config".to_string(), self.r#error_handling_config.to_pulumi_value().await);
            map.insert("intermediate_bucket_name".to_string(), self.r#intermediate_bucket_name.to_pulumi_value().await);
            map.insert("object".to_string(), self.r#object.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflake {
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
                    r#bucket_prefix: {
                        let field_value = match fields_map.get("bucket_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#error_handling_config: {
                        let field_value = match fields_map.get("error_handling_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_handling_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflakeErrorHandlingConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#intermediate_bucket_name: {
                        let field_value = match fields_map.get("intermediate_bucket_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'intermediate_bucket_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#object: {
                        let field_value = match fields_map.get("object") {
                            Some(value) => value,
                            None => bail!("Missing field 'object' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
