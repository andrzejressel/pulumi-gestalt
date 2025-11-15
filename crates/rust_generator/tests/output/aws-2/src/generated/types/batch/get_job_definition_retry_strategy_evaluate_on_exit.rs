#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionRetryStrategyEvaluateOnExit {
    /// Specifies the action to take if all of the specified conditions (onStatusReason, onReason, and onExitCode) are met. The values aren't case sensitive.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// Contains a glob pattern to match against the decimal representation of the ExitCode returned for a job.
    #[builder(into)]
    #[serde(rename = "onExitCode")]
    pub r#on_exit_code: String,
    /// Contains a glob pattern to match against the Reason returned for a job.
    #[builder(into)]
    #[serde(rename = "onReason")]
    pub r#on_reason: String,
    /// Contains a glob pattern to match against the StatusReason returned for a job.
    #[builder(into)]
    #[serde(rename = "onStatusReason")]
    pub r#on_status_reason: String,
}
