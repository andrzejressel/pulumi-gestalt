#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTaskSpecRestartPolicy {
    /// Condition for restart
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Option<String>,
    /// Delay between restart attempts (ms|s|m|h)
    #[builder(into)]
    #[serde(rename = "delay")]
    pub r#delay: Option<String>,
    /// Maximum attempts to restart a given container before giving up (default value is `0`, which is ignored)
    #[builder(into)]
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: Option<i32>,
    /// The time window used to evaluate the restart policy (default value is `0`, which is unbounded) (ms|s|m|h)
    #[builder(into)]
    #[serde(rename = "window")]
    pub r#window: Option<String>,
}
