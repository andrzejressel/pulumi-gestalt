#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HciDeploymentSettingScaleUnitInfrastructureNetwork {
    /// Whether DHCP is enabled for hosts and cluster IPs. Possible values are `true` and `false`. defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    /// 
    /// > **NOTE:** If `dhcp_enabled` is set to `false`, the deployment will use static IPs. If set to `true`, the gateway and DNS servers are not required.
    #[builder(into)]
    #[serde(rename = "dhcpEnabled")]
    pub r#dhcp_enabled: Option<bool>,
    /// Specifies a list of IPv4 addresses of the DNS servers in your environment. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Vec<String>,
    /// Specifies the default gateway that should be used for the provided IP address space. It should be in the format of an IPv4 IP address. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "gateway")]
    pub r#gateway: String,
    /// One or more `ip_pool` blocks as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "ipPools")]
    pub r#ip_pools: Vec<super::super::types::stack::HciDeploymentSettingScaleUnitInfrastructureNetworkIpPool>,
    /// Specifies the subnet mask that matches the provided IP address space. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "subnetMask")]
    pub r#subnet_mask: String,
}
