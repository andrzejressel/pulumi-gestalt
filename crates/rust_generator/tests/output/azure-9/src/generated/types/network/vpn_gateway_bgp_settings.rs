#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpnGatewayBgpSettings {
    /// The ASN of the BGP Speaker. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: i32,
    /// The Address which should be used for the BGP Peering.
    #[builder(into)]
    #[serde(rename = "bgpPeeringAddress")]
    pub r#bgp_peering_address: Option<String>,
    /// An `instance_bgp_peering_address` block as defined below.
    #[builder(into)]
    #[serde(rename = "instance0BgpPeeringAddress")]
    pub r#instance_0_bgp_peering_address: Option<Box<super::super::types::network::VpnGatewayBgpSettingsInstance0BgpPeeringAddress>>,
    /// An `instance_bgp_peering_address` block as defined below.
    #[builder(into)]
    #[serde(rename = "instance1BgpPeeringAddress")]
    pub r#instance_1_bgp_peering_address: Option<Box<super::super::types::network::VpnGatewayBgpSettingsInstance1BgpPeeringAddress>>,
    /// The weight added to Routes learned from this BGP Speaker. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "peerWeight")]
    pub r#peer_weight: i32,
}
