#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceRollbackConfig {
    /// Delay between task rollbacks (ns|us|ms|s|m|h). Defaults to `0s`.
    #[builder(into)]
    #[serde(rename = "delay")]
    pub r#delay: Option<String>,
    /// Action on rollback failure: pause | continue. Defaults to `pause`.
    #[builder(into)]
    #[serde(rename = "failureAction")]
    pub r#failure_action: Option<String>,
    /// Failure rate to tolerate during a rollback. Defaults to `0.0`.
    #[builder(into)]
    #[serde(rename = "maxFailureRatio")]
    pub r#max_failure_ratio: Option<String>,
    /// Duration after each task rollback to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`.
    #[builder(into)]
    #[serde(rename = "monitor")]
    pub r#monitor: Option<String>,
    /// Rollback order: either 'stop-first' or 'start-first'. Defaults to `stop-first`.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<String>,
    /// Maximum number of tasks to be rollbacked in one iteration. Defaults to `1`
    #[builder(into)]
    #[serde(rename = "parallelism")]
    pub r#parallelism: Option<i32>,
}
