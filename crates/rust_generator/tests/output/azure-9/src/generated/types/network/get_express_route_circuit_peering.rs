#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetExpressRouteCircuitPeering {
    /// The Either a 16-bit or a 32-bit ASN for Azure.
    #[builder(into)]
    #[serde(rename = "azureAsn")]
    pub r#azure_asn: i32,
    /// The Either a 16-bit or a 32-bit ASN. Can either be public or private.
    #[builder(into)]
    #[serde(rename = "peerAsn")]
    pub r#peer_asn: i32,
    /// The type of the ExpressRoute Circuit Peering. Acceptable values include `AzurePrivatePeering`, `AzurePublicPeering` and `MicrosoftPeering`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "peeringType")]
    pub r#peering_type: String,
    /// A `/30` subnet for the primary link.
    #[builder(into)]
    #[serde(rename = "primaryPeerAddressPrefix")]
    pub r#primary_peer_address_prefix: String,
    /// A `/30` subnet for the secondary link.
    #[builder(into)]
    #[serde(rename = "secondaryPeerAddressPrefix")]
    pub r#secondary_peer_address_prefix: String,
    /// The shared key. Can be a maximum of 25 characters.
    #[builder(into)]
    #[serde(rename = "sharedKey")]
    pub r#shared_key: String,
    /// A valid VLAN ID to establish this peering on.
    #[builder(into)]
    #[serde(rename = "vlanId")]
    pub r#vlan_id: i32,
}
