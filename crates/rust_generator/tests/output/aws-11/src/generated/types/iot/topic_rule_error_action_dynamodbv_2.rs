#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicRuleErrorActionDynamodbv2 {
    /// Configuration block with DynamoDB Table to which the message will be written. Nested arguments below.
    #[builder(into)]
    #[serde(rename = "putItem")]
    pub r#put_item: Option<Box<super::super::types::iot::TopicRuleErrorActionDynamodbv2PutItem>>,
    /// The ARN of the IAM role that grants access to the DynamoDB table.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
}
