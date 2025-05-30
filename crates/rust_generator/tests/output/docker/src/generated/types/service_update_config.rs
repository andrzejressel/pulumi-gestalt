#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceUpdateConfig {
    /// Delay between task updates `(ns|us|ms|s|m|h)`. Defaults to `0s`.
    #[builder(into, default)]
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    /// Action on update failure: `pause`, `continue` or `rollback`. Defaults to `pause`.
    #[builder(into, default)]
    #[serde(rename = "failureAction")]
    pub r#failure_action: Box<Option<String>>,
    /// Failure rate to tolerate during an update. Defaults to `0.0`.
    #[builder(into, default)]
    #[serde(rename = "maxFailureRatio")]
    pub r#max_failure_ratio: Box<Option<String>>,
    /// Duration after each task update to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`.
    #[builder(into, default)]
    #[serde(rename = "monitor")]
    pub r#monitor: Box<Option<String>>,
    /// Update order: either 'stop-first' or 'start-first'. Defaults to `stop-first`.
    #[builder(into, default)]
    #[serde(rename = "order")]
    pub r#order: Box<Option<String>>,
    /// Maximum number of tasks to be updated in one iteration. Defaults to `1`
    #[builder(into, default)]
    #[serde(rename = "parallelism")]
    pub r#parallelism: Box<Option<i32>>,
}
