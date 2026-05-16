#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeLogConfigurationCloudwatchLogsLogDestination {
    /// Amazon Web Services Resource Name (ARN) for the CloudWatch log group to which EventBridge sends the log records.
    #[builder(into)]
    #[serde(rename = "logGroupArn")]
    pub r#log_group_arn: String,
}
