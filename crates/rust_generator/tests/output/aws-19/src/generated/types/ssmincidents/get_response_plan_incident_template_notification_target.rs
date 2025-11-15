#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetResponsePlanIncidentTemplateNotificationTarget {
    /// The ARN of the Amazon SNS topic.
    #[builder(into)]
    #[serde(rename = "snsTopicArn")]
    pub r#sns_topic_arn: String,
}
