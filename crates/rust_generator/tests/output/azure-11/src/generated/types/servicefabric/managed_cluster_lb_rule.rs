#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedClusterLbRule {
    /// LB Backend port.
    #[builder(into)]
    #[serde(rename = "backendPort")]
    pub r#backend_port: i32,
    /// LB Frontend port.
    #[builder(into)]
    #[serde(rename = "frontendPort")]
    pub r#frontend_port: i32,
    /// Protocol for the probe. Can be one of `tcp`, `udp`, `http`, or `https`.
    #[builder(into)]
    #[serde(rename = "probeProtocol")]
    pub r#probe_protocol: String,
    /// Path for the probe to check, when probe protocol is set to `http`.
    #[builder(into)]
    #[serde(rename = "probeRequestPath")]
    pub r#probe_request_path: Option<String>,
    /// The transport protocol used in this rule. Can be one of `tcp` or `udp`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
}
