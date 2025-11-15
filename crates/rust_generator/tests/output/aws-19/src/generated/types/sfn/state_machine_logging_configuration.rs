#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StateMachineLoggingConfiguration {
    /// Determines whether execution data is included in your log. When set to `false`, data is excluded.
    #[builder(into)]
    #[serde(rename = "includeExecutionData")]
    pub r#include_execution_data: Option<bool>,
    /// Defines which category of execution history events are logged. Valid values: `ALL`, `ERROR`, `FATAL`, `OFF`
    #[builder(into)]
    #[serde(rename = "level")]
    pub r#level: Option<String>,
    /// Amazon Resource Name (ARN) of a CloudWatch log group. Make sure the State Machine has the correct IAM policies for logging. The ARN must end with `:*`
    #[builder(into)]
    #[serde(rename = "logDestination")]
    pub r#log_destination: Option<String>,
}
