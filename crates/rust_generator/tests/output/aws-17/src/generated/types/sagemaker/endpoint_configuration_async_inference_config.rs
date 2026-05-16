#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointConfigurationAsyncInferenceConfig {
    /// Configures the behavior of the client used by Amazon SageMaker to interact with the model container during asynchronous inference.
    #[builder(into)]
    #[serde(rename = "clientConfig")]
    pub r#client_config: Option<Box<super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfigClientConfig>>,
    /// Specifies the configuration for asynchronous inference invocation outputs.
    #[builder(into)]
    #[serde(rename = "outputConfig")]
    pub r#output_config: Box<super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfigOutputConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointConfigurationAsyncInferenceConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("client_config".to_string(), self.r#client_config.to_pulumi_value().await);
            map.insert("output_config".to_string(), self.r#output_config.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointConfigurationAsyncInferenceConfig {
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
                    r#client_config: {
                        let field_value = match fields_map.get("client_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfigClientConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_config: {
                        let field_value = match fields_map.get("output_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfigOutputConfig> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
