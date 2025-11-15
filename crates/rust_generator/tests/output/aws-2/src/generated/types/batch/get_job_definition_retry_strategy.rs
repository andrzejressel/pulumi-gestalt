#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionRetryStrategy {
    /// The number of times to move a job to the RUNNABLE status.
    #[builder(into)]
    #[serde(rename = "attempts")]
    pub r#attempts: i32,
    /// Array of up to 5 objects that specify the conditions where jobs are retried or failed.
    #[builder(into)]
    #[serde(rename = "evaluateOnExits")]
    pub r#evaluate_on_exits: Vec<super::super::types::batch::GetJobDefinitionRetryStrategyEvaluateOnExit>,
}
