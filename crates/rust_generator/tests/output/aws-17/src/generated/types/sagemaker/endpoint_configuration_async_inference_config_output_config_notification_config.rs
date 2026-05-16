#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointConfigurationAsyncInferenceConfigOutputConfigNotificationConfig {
    /// Amazon SNS topic to post a notification to when inference fails. If no topic is provided, no notification is sent on failure.
    #[builder(into)]
    #[serde(rename = "errorTopic")]
    pub r#error_topic: Option<String>,
    /// The Amazon SNS topics where you want the inference response to be included. Valid values are `SUCCESS_NOTIFICATION_TOPIC` and `ERROR_NOTIFICATION_TOPIC`.
    #[builder(into)]
    #[serde(rename = "includeInferenceResponseIns")]
    pub r#include_inference_response_ins: Option<Vec<String>>,
    /// Amazon SNS topic to post a notification to when inference completes successfully. If no topic is provided, no notification is sent on success.
    #[builder(into)]
    #[serde(rename = "successTopic")]
    pub r#success_topic: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointConfigurationAsyncInferenceConfigOutputConfigNotificationConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("error_topic".to_string(), self.r#error_topic.to_pulumi_value().await);
            map.insert("include_inference_response_ins".to_string(), self.r#include_inference_response_ins.to_pulumi_value().await);
            map.insert("success_topic".to_string(), self.r#success_topic.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointConfigurationAsyncInferenceConfigOutputConfigNotificationConfig {
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
                    r#error_topic: {
                        let field_value = match fields_map.get("error_topic") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_topic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#include_inference_response_ins: {
                        let field_value = match fields_map.get("include_inference_response_ins") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_inference_response_ins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#success_topic: {
                        let field_value = match fields_map.get("success_topic") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_topic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
