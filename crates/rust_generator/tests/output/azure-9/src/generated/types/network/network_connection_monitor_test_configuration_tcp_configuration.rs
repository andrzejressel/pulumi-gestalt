#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkConnectionMonitorTestConfigurationTcpConfiguration {
    /// The destination port behavior for the TCP connection. Possible values are `None` and `ListenIfAvailable`.
    #[builder(into)]
    #[serde(rename = "destinationPortBehavior")]
    pub r#destination_port_behavior: Option<String>,
    /// The port for the TCP connection.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// Should path evaluation with trace route be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "traceRouteEnabled")]
    pub r#trace_route_enabled: Option<bool>,
}
