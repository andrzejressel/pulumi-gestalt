#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NextGenerationFirewallVirtualNetworkPanoramaNetworkProfileVnetConfiguration {
    #[builder(into, default)]
    #[serde(rename = "ipOfTrustForUserDefinedRoutes")]
    pub r#ip_of_trust_for_user_defined_routes: Box<Option<String>>,
    /// The ID of the Trust subnet.
    #[builder(into, default)]
    #[serde(rename = "trustedSubnetId")]
    pub r#trusted_subnet_id: Box<Option<String>>,
    /// The ID of the UnTrust subnet.
    #[builder(into, default)]
    #[serde(rename = "untrustedSubnetId")]
    pub r#untrusted_subnet_id: Box<Option<String>>,
    /// The ID of the Virtual Network.
    #[builder(into)]
    #[serde(rename = "virtualNetworkId")]
    pub r#virtual_network_id: Box<String>,
}
