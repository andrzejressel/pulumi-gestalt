#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobQueueJobStateTimeLimitAction {
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    #[builder(into)]
    #[serde(rename = "maxTimeSeconds")]
    pub r#max_time_seconds: i32,
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: String,
    /// Describes the ability of the queue to accept new jobs (for example, `ENABLED` or `DISABLED`).
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
}
