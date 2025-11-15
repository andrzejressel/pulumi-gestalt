#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobManualTriggerConfig {
    /// Number of parallel replicas of a job that can run at a given time.
    #[builder(into)]
    #[serde(rename = "parallelism")]
    pub r#parallelism: Option<i32>,
    /// Minimum number of successful replica completions before overall job completion.
    #[builder(into)]
    #[serde(rename = "replicaCompletionCount")]
    pub r#replica_completion_count: Option<i32>,
}
