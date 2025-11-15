#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicRuleErrorActionSns {
    /// The message format of the message to publish. Accepted values are "JSON" and "RAW".
    #[builder(into)]
    #[serde(rename = "messageFormat")]
    pub r#message_format: Option<String>,
    /// The ARN of the IAM role that grants access.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The ARN of the SNS topic.
    #[builder(into)]
    #[serde(rename = "targetArn")]
    pub r#target_arn: String,
}
