#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTaskSpecContainerSpecHealthcheck {
    /// Time between running the check (ms|s|m|h). Defaults to `0s`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Option<String>,
    /// Consecutive failures needed to report unhealthy. Defaults to `0`
    #[builder(into)]
    #[serde(rename = "retries")]
    pub r#retries: Option<i32>,
    /// Start period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`.
    #[builder(into)]
    #[serde(rename = "startPeriod")]
    pub r#start_period: Option<String>,
    /// The test to perform as list
    #[builder(into)]
    #[serde(rename = "tests")]
    pub r#tests: Vec<String>,
    /// Maximum time to allow one check to run (ms|s|m|h). Defaults to `0s`.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}
