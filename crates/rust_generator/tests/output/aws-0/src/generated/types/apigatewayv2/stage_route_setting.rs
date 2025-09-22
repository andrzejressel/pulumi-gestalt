#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StageRouteSetting {
    /// Whether data trace logging is enabled for the route. Affects the log entries pushed to Amazon CloudWatch Logs.
    /// Defaults to `false`. Supported only for WebSocket APIs.
    #[builder(into)]
    #[serde(rename = "dataTraceEnabled")]
    pub r#data_trace_enabled: Option<bool>,
    /// Whether detailed metrics are enabled for the route. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "detailedMetricsEnabled")]
    pub r#detailed_metrics_enabled: Option<bool>,
    /// Logging level for the route. Affects the log entries pushed to Amazon CloudWatch Logs.
    /// Valid values: `ERROR`, `INFO`, `OFF`. Defaults to `OFF`. Supported only for WebSocket APIs. This provider will only perform drift detection of its value when present in a configuration.
    #[builder(into)]
    #[serde(rename = "loggingLevel")]
    pub r#logging_level: Option<String>,
    /// Route key.
    #[builder(into)]
    #[serde(rename = "routeKey")]
    pub r#route_key: String,
    /// Throttling burst limit for the route.
    #[builder(into)]
    #[serde(rename = "throttlingBurstLimit")]
    pub r#throttling_burst_limit: Option<i32>,
    /// Throttling rate limit for the route.
    #[builder(into)]
    #[serde(rename = "throttlingRateLimit")]
    pub r#throttling_rate_limit: Option<f64>,
}
