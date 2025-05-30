#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZeroTrustSplitTunnelTunnel {
    /// The address for the tunnel.
    #[builder(into, default)]
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    /// A description for the tunnel.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The domain name for the tunnel.
    #[builder(into, default)]
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
}
