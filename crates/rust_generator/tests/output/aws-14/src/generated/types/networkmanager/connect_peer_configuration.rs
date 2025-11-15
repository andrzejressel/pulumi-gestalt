#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectPeerConfiguration {
    #[builder(into)]
    #[serde(rename = "bgpConfigurations")]
    pub r#bgp_configurations: Option<Vec<super::super::types::networkmanager::ConnectPeerConfigurationBgpConfiguration>>,
    /// A Connect peer core network address.
    #[builder(into)]
    #[serde(rename = "coreNetworkAddress")]
    pub r#core_network_address: Option<String>,
    /// The inside IP addresses used for BGP peering. Required when the Connect attachment protocol is `GRE`. See `aws.networkmanager.ConnectAttachment` for details.
    #[builder(into)]
    #[serde(rename = "insideCidrBlocks")]
    pub r#inside_cidr_blocks: Option<Vec<String>>,
    /// The Connect peer address.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "peerAddress")]
    pub r#peer_address: Option<String>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
}
