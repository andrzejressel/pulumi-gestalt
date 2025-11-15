#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallManagementIpConfiguration {
    /// Specifies the name of the IP Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The private IP address associated with the Firewall.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Option<String>,
    /// The ID of the Public IP Address associated with the firewall.
    /// 
    /// > **NOTE** The Public IP must have a `Static` allocation and `Standard` SKU.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: String,
    /// Reference to the subnet associated with the IP Configuration. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE** The Management Subnet used for the Firewall must have the name `AzureFirewallManagementSubnet` and the subnet mask must be at least a `/26`.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}
