#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualMachineScaleSetNetworkInterface {
    /// An array of the DNS servers in use.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Vec<String>,
    /// Whether to enable accelerated networking or not.
    #[builder(into)]
    #[serde(rename = "enableAcceleratedNetworking")]
    pub r#enable_accelerated_networking: bool,
    /// Whether IP forwarding is enabled on this NIC.
    #[builder(into)]
    #[serde(rename = "enableIpForwarding")]
    pub r#enable_ip_forwarding: bool,
    /// An `ip_configuration` block as documented below.
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Vec<super::super::types::compute::GetVirtualMachineScaleSetNetworkInterfaceIpConfiguration>,
    /// The name of this Virtual Machine Scale Set.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The identifier for the network security group.
    #[builder(into)]
    #[serde(rename = "networkSecurityGroupId")]
    pub r#network_security_group_id: String,
    /// If this ip_configuration is the primary one.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: bool,
}
