#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReceiptRuleStopAction {
    /// The position of the action in the receipt rule
    #[builder(into)]
    #[serde(rename = "position")]
    pub r#position: Box<i32>,
    /// The scope to apply. The only acceptable value is `RuleSet`.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Box<String>,
    /// The ARN of an SNS topic to notify
    #[builder(into, default)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: Box<Option<String>>,
}
