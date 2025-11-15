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
