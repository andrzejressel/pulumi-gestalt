#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicRuleErrorActionCloudwatchMetric {
    /// The CloudWatch metric name.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: String,
    /// The CloudWatch metric namespace name.
    #[builder(into)]
    #[serde(rename = "metricNamespace")]
    pub r#metric_namespace: String,
    /// An optional Unix timestamp (http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/cloudwatch_concepts.html#about_timestamp).
    #[builder(into)]
    #[serde(rename = "metricTimestamp")]
    pub r#metric_timestamp: Option<String>,
    /// The metric unit (supported units can be found here: http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/cloudwatch_concepts.html#Unit)
    #[builder(into)]
    #[serde(rename = "metricUnit")]
    pub r#metric_unit: String,
    /// The CloudWatch metric value.
    #[builder(into)]
    #[serde(rename = "metricValue")]
    pub r#metric_value: String,
    /// The IAM role ARN that allows access to the CloudWatch metric.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
}
