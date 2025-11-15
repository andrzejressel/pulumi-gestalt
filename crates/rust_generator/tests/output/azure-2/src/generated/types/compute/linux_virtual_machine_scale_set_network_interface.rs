#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxVirtualMachineScaleSetNetworkInterface {
    /// A list of IP Addresses of DNS Servers which should be assigned to the Network Interface.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Option<Vec<String>>,
    /// Does this Network Interface support Accelerated Networking? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enableAcceleratedNetworking")]
    pub r#enable_accelerated_networking: Option<bool>,
    /// Does this Network Interface support IP Forwarding? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enableIpForwarding")]
    pub r#enable_ip_forwarding: Option<bool>,
    /// One or more `ip_configuration` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Vec<super::super::types::compute::LinuxVirtualMachineScaleSetNetworkInterfaceIpConfiguration>,
    /// The Name which should be used for this Network Interface. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The ID of a Network Security Group which should be assigned to this Network Interface.
    #[builder(into)]
    #[serde(rename = "networkSecurityGroupId")]
    pub r#network_security_group_id: Option<String>,
    /// Is this the Primary IP Configuration?
    /// 
    /// > **Note:** If multiple `network_interface` blocks are specified, one must be set to `primary`.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Option<bool>,
}
