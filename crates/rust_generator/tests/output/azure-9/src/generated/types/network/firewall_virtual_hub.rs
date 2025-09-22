#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallVirtualHub {
    /// The private IP address associated with the Firewall.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Option<String>,
    /// The list of public IP addresses associated with the Firewall.
    #[builder(into)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Option<Vec<String>>,
    /// Specifies the number of public IPs to assign to the Firewall. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "publicIpCount")]
    pub r#public_ip_count: Option<i32>,
    /// Specifies the ID of the Virtual Hub where the Firewall resides in.
    #[builder(into)]
    #[serde(rename = "virtualHubId")]
    pub r#virtual_hub_id: String,
}
