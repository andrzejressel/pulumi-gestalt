#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketNotificationLambdaFunction {
    /// [Event](http://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html#notification-how-to-event-types-and-destinations) for which to send notifications.
    #[builder(into)]
    #[serde(rename = "events")]
    pub r#events: Vec<String>,
    /// Object key name prefix.
    #[builder(into)]
    #[serde(rename = "filterPrefix")]
    pub r#filter_prefix: Option<String>,
    /// Object key name suffix.
    #[builder(into)]
    #[serde(rename = "filterSuffix")]
    pub r#filter_suffix: Option<String>,
    /// Unique identifier for each of the notification configurations.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Lambda function ARN.
    #[builder(into)]
    #[serde(rename = "lambdaFunctionArn")]
    pub r#lambda_function_arn: Option<String>,
}
