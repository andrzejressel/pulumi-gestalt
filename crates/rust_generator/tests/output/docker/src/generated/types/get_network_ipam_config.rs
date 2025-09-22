#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkIpamConfig {
    /// Auxiliary IPv4 or IPv6 addresses used by Network driver
    #[builder(into)]
    #[serde(rename = "auxAddress")]
    pub r#aux_address: Option<std::collections::HashMap<String, String>>,
    /// The IP address of the gateway
    #[builder(into)]
    #[serde(rename = "gateway")]
    pub r#gateway: Option<String>,
    /// The ip range in CIDR form
    #[builder(into)]
    #[serde(rename = "ipRange")]
    pub r#ip_range: Option<String>,
    /// The subnet in CIDR form
    #[builder(into)]
    #[serde(rename = "subnet")]
    pub r#subnet: Option<String>,
}
