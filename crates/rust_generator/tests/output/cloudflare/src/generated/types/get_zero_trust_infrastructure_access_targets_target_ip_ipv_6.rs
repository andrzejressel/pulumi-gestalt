#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetZeroTrustInfrastructureAccessTargetsTargetIpIpv6 {
    /// The IP address of the target.
    #[builder(into)]
    #[serde(rename = "ipAddr")]
    pub r#ip_addr: String,
    /// The private virtual network identifier for the target.
    #[builder(into)]
    #[serde(rename = "virtualNetworkId")]
    pub r#virtual_network_id: String,
}
