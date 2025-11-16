#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLoadBalancerPoolsPoolLoadShedding {
    /// Percent of traffic to shed 0 - 100.
    #[builder(into)]
    #[serde(rename = "defaultPercent")]
    pub r#default_percent: Option<f64>,
    /// Method of shedding traffic. Available values: `""`, `hash`, `random`
    #[builder(into)]
    #[serde(rename = "defaultPolicy")]
    pub r#default_policy: Option<String>,
    /// Percent of session traffic to shed 0 - 100.
    #[builder(into)]
    #[serde(rename = "sessionPercent")]
    pub r#session_percent: Option<f64>,
    /// Method of shedding traffic. Available values: `""`, `hash`
    #[builder(into)]
    #[serde(rename = "sessionPolicy")]
    pub r#session_policy: Option<String>,
}
