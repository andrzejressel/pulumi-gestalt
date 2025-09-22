#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NextGenerationFirewallVirtualNetworkLocalRulestackDestinationNatBackendConfig {
    /// The port number to send traffic to.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// The IP Address to send the traffic to.
    #[builder(into)]
    #[serde(rename = "publicIpAddress")]
    pub r#public_ip_address: String,
}
