#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkspaceLoggingConfiguration {
    /// The ARN of the CloudWatch log group to which the vended log data will be published. This log group must exist.
    #[builder(into)]
    #[serde(rename = "logGroupArn")]
    pub r#log_group_arn: Box<String>,
}
