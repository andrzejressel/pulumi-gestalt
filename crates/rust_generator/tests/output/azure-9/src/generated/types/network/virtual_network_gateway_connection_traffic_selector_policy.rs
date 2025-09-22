#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNetworkGatewayConnectionTrafficSelectorPolicy {
    /// List of local CIDRs.
    #[builder(into)]
    #[serde(rename = "localAddressCidrs")]
    pub r#local_address_cidrs: Vec<String>,
    /// List of remote CIDRs.
    #[builder(into)]
    #[serde(rename = "remoteAddressCidrs")]
    pub r#remote_address_cidrs: Vec<String>,
}
