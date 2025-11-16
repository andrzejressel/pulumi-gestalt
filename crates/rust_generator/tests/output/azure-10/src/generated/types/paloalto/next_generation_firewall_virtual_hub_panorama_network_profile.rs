#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NextGenerationFirewallVirtualHubPanoramaNetworkProfile {
    #[builder(into)]
    #[serde(rename = "egressNatIpAddressIds")]
    pub r#egress_nat_ip_address_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "egressNatIpAddresses")]
    pub r#egress_nat_ip_addresses: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "ipOfTrustForUserDefinedRoutes")]
    pub r#ip_of_trust_for_user_defined_routes: Option<String>,
    #[builder(into)]
    #[serde(rename = "networkVirtualApplianceId")]
    pub r#network_virtual_appliance_id: String,
    #[builder(into)]
    #[serde(rename = "publicIpAddressIds")]
    pub r#public_ip_address_ids: Vec<String>,
    #[builder(into)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "trustedAddressRanges")]
    pub r#trusted_address_ranges: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "trustedSubnetId")]
    pub r#trusted_subnet_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "untrustedSubnetId")]
    pub r#untrusted_subnet_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "virtualHubId")]
    pub r#virtual_hub_id: String,
}
