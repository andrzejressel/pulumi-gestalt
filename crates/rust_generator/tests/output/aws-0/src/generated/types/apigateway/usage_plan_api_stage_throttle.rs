#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UsagePlanApiStageThrottle {
    /// The API request burst limit, the maximum rate limit over a time ranging from one to a few seconds, depending upon whether the underlying token bucket is at its full capacity.
    #[builder(into)]
    #[serde(rename = "burstLimit")]
    pub r#burst_limit: Option<i32>,
    /// Method to apply the throttle settings for. Specfiy the path and method, for example `/test/GET`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// The API request steady-state rate limit.
    #[builder(into)]
    #[serde(rename = "rateLimit")]
    pub r#rate_limit: Option<f64>,
}
