#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ContainerNetworkData {
    /// The network gateway of the container.
    #[builder(into)]
    #[serde(rename = "gateway")]
    pub r#gateway: Option<String>,
    /// The IPV6 address of the container.
    #[builder(into)]
    #[serde(rename = "globalIpv6Address")]
    pub r#global_ipv_6_address: Option<String>,
    /// The IPV6 prefix length address of the container.
    #[builder(into)]
    #[serde(rename = "globalIpv6PrefixLength")]
    pub r#global_ipv_6_prefix_length: Option<i32>,
    /// The IP address of the container.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Option<String>,
    /// The IP prefix length of the container.
    #[builder(into)]
    #[serde(rename = "ipPrefixLength")]
    pub r#ip_prefix_length: Option<i32>,
    /// The IPV6 gateway of the container.
    #[builder(into)]
    #[serde(rename = "ipv6Gateway")]
    pub r#ipv_6_gateway: Option<String>,
    /// The MAC address of the container.
    #[builder(into)]
    #[serde(rename = "macAddress")]
    pub r#mac_address: Option<String>,
    /// The name of the network
    #[builder(into)]
    #[serde(rename = "networkName")]
    pub r#network_name: Option<String>,
}
