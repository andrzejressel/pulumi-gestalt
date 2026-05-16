#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCloudFormationTypeLoggingConfig {
    /// Name of the CloudWatch Log Group where CloudFormation sends error logging information when invoking the type's handlers.
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: String,
    /// ARN of the IAM Role CloudFormation assumes when sending error logging information to CloudWatch Logs.
    #[builder(into)]
    #[serde(rename = "logRoleArn")]
    pub r#log_role_arn: String,
}
