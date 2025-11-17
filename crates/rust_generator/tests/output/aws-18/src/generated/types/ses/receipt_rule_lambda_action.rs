#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReceiptRuleLambdaAction {
    /// The ARN of the Lambda function to invoke
    #[builder(into)]
    #[serde(rename = "functionArn")]
    pub r#function_arn: String,
    /// `Event` or `RequestResponse`
    #[builder(into)]
    #[serde(rename = "invocationType")]
    pub r#invocation_type: Option<String>,
    /// The position of the action in the receipt rule
    #[builder(into)]
    #[serde(rename = "position")]
    pub r#position: i32,
    /// The ARN of an SNS topic to notify
    #[builder(into)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: Option<String>,
}
