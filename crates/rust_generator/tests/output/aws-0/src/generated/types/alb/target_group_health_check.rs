#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TargetGroupHealthCheck {
    /// Whether health checks are enabled. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Number of consecutive health check successes required before considering a target healthy. The range is 2-10. Defaults to 3.
    #[builder(into)]
    #[serde(rename = "healthyThreshold")]
    pub r#healthy_threshold: Option<i32>,
    /// Approximate amount of time, in seconds, between health checks of an individual target. The range is 5-300. For `lambda` target groups, it needs to be greater than the timeout of the underlying `lambda`. Defaults to 30.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Option<i32>,
    /// The HTTP or gRPC codes to use when checking for a successful response from a target.
    /// The `health_check.protocol` must be one of `HTTP` or `HTTPS` or the `target_type` must be `lambda`.
    /// Values can be comma-separated individual values (e.g., "200,202") or a range of values (e.g., "200-299").
    /// * For gRPC-based target groups (i.e., the `protocol` is one of `HTTP` or `HTTPS` and the `protocol_version` is `GRPC`), values can be between `0` and `99`. The default is `12`.
    /// * When used with an Application Load Balancer (i.e., the `protocol` is one of `HTTP` or `HTTPS` and the `protocol_version` is not `GRPC`), values can be between `200` and `499`. The default is `200`.
    /// * When used with a Network Load Balancer (i.e., the `protocol` is one of `TCP`, `TCP_UDP`, `UDP`, or `TLS`), values can be between `200` and `599`. The default is `200-399`.
    /// * When the `target_type` is `lambda`, values can be between `200` and `499`. The default is `200`.
    #[builder(into)]
    #[serde(rename = "matcher")]
    pub r#matcher: Option<String>,
    /// Destination for the health check request. Required for HTTP/HTTPS ALB and HTTP NLB. Only applies to HTTP/HTTPS.
    /// * For HTTP and HTTPS health checks, the default is `/`.
    /// * For gRPC health checks, the default is `/AWS.ALB/healthcheck`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// The port the load balancer uses when performing health checks on targets.
    /// Valid values are either `traffic-port`, to use the same port as the target group, or a valid port number between `1` and `65536`.
    /// Default is `traffic-port`.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<String>,
    /// Protocol the load balancer uses when performing health checks on targets.
    /// Must be one of `TCP`, `HTTP`, or `HTTPS`.
    /// The `TCP` protocol is not supported for health checks if the protocol of the target group is `HTTP` or `HTTPS`.
    /// Default is `HTTP`.
    /// Cannot be specified when the `target_type` is `lambda`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    /// Amount of time, in seconds, during which no response from a target means a failed health check. The range is 2–120 seconds. For target groups with a protocol of HTTP, the default is 6 seconds. For target groups with a protocol of TCP, TLS or HTTPS, the default is 10 seconds. For target groups with a protocol of GENEVE, the default is 5 seconds. If the target type is lambda, the default is 30 seconds.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<i32>,
    /// Number of consecutive health check failures required before considering a target unhealthy. The range is 2-10. Defaults to 3.
    #[builder(into)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: Option<i32>,
}
