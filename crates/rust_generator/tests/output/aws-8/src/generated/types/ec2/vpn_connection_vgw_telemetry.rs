#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnConnectionVgwTelemetry {
    /// The number of accepted routes.
    #[builder(into)]
    #[serde(rename = "acceptedRouteCount")]
    pub r#accepted_route_count: Option<i32>,
    /// The Amazon Resource Name (ARN) of the VPN tunnel endpoint certificate.
    #[builder(into)]
    #[serde(rename = "certificateArn")]
    pub r#certificate_arn: Option<String>,
    /// The date and time of the last change in status.
    #[builder(into)]
    #[serde(rename = "lastStatusChange")]
    pub r#last_status_change: Option<String>,
    /// The Internet-routable IP address of the virtual private gateway's outside interface.
    #[builder(into)]
    #[serde(rename = "outsideIpAddress")]
    pub r#outside_ip_address: Option<String>,
    /// The status of the VPN tunnel.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// If an error occurs, a description of the error.
    #[builder(into)]
    #[serde(rename = "statusMessage")]
    pub r#status_message: Option<String>,
}
