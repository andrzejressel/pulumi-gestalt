#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpnConnectionDetail {
    /// (Output)
    /// The Cloud Router info.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudRouters")]
    pub r#cloud_routers: Option<Vec<super::super::types::edgecontainer::VpnConnectionDetailCloudRouter>>,
    /// (Output)
    /// Each connection has multiple Cloud VPN gateways.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudVpns")]
    pub r#cloud_vpns: Option<Vec<super::super::types::edgecontainer::VpnConnectionDetailCloudVpn>>,
    /// (Output)
    /// The error message. This is only populated when state=ERROR.
    #[builder(into)]
    #[serde(rename = "error")]
    pub r#error: Option<String>,
    /// (Output)
    /// The current connection state.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}
