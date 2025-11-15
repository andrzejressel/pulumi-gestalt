#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkInterfaceIpConfiguration {
    /// The Frontend IP Configuration ID of a Gateway SKU Load Balancer.
    #[builder(into)]
    #[serde(rename = "gatewayLoadBalancerFrontendIpConfigurationId")]
    pub r#gateway_load_balancer_frontend_ip_configuration_id: Option<String>,
    /// A name used for this IP Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Is this the Primary IP Configuration? Must be `true` for the first `ip_configuration` when multiple are specified. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Option<bool>,
    /// The first private IP address of the network interface.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Option<String>,
    /// The allocation method used for the Private IP Address. Possible values are `Dynamic` and `Static`.
    /// 
    /// > **Note:** `Dynamic` means "An IP is automatically assigned during creation of this Network Interface"; `Static` means "User supplied IP address will be used"
    #[builder(into)]
    #[serde(rename = "privateIpAddressAllocation")]
    pub r#private_ip_address_allocation: String,
    /// The IP Version to use. Possible values are `IPv4` or `IPv6`. Defaults to `IPv4`.
    #[builder(into)]
    #[serde(rename = "privateIpAddressVersion")]
    pub r#private_ip_address_version: Option<String>,
    /// Reference to a Public IP Address to associate with this NIC
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Option<String>,
    /// The ID of the Subnet where this Network Interface should be located in.
    /// 
    /// > **Note:** This is required when `private_ip_address_version` is set to `IPv4`.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
}
