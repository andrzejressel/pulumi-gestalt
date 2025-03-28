#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterNetworkProfileNatGatewayProfile {
    /// The outcome (resource IDs) of the specified arguments.
    #[builder(into, default)]
    #[serde(rename = "effectiveOutboundIps")]
    pub r#effective_outbound_ips: Box<Option<Vec<String>>>,
    /// Desired outbound flow idle timeout in minutes for the managed nat gateway. Must be between `4` and `120` inclusive. Defaults to `4`.
    #[builder(into, default)]
    #[serde(rename = "idleTimeoutInMinutes")]
    pub r#idle_timeout_in_minutes: Box<Option<i32>>,
    /// Count of desired managed outbound IPs for the managed nat gateway. Must be between `1` and `16` inclusive.
    #[builder(into, default)]
    #[serde(rename = "managedOutboundIpCount")]
    pub r#managed_outbound_ip_count: Box<Option<i32>>,
}
