#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectPeerConfigurationBgpConfiguration {
    /// A Connect peer core network address.
    #[builder(into)]
    #[serde(rename = "coreNetworkAddress")]
    pub r#core_network_address: Option<String>,
    #[builder(into)]
    #[serde(rename = "coreNetworkAsn")]
    pub r#core_network_asn: Option<i32>,
    /// The Connect peer address.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "peerAddress")]
    pub r#peer_address: Option<String>,
    #[builder(into)]
    #[serde(rename = "peerAsn")]
    pub r#peer_asn: Option<i32>,
}
