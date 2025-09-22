#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicRuleStepFunction {
    /// The prefix used to generate, along with a UUID, the unique state machine execution name.
    #[builder(into)]
    #[serde(rename = "executionNamePrefix")]
    pub r#execution_name_prefix: Option<String>,
    /// The ARN of the IAM role that grants access to start execution of the state machine.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The name of the Step Functions state machine whose execution will be started.
    #[builder(into)]
    #[serde(rename = "stateMachineName")]
    pub r#state_machine_name: String,
}
