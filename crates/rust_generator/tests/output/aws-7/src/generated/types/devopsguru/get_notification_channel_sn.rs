#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNotificationChannelSn {
    /// Amazon Resource Name (ARN) of an Amazon Simple Notification Service topic.
    #[builder(into)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: String,
}
