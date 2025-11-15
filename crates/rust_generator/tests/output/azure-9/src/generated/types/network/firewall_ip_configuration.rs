#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallIpConfiguration {
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
    /// > **NOTE** A public ip address is required unless a `management_ip_configuration` block is specified.
    /// 
    /// > **NOTE** When multiple `ip_configuration` blocks with `public_ip_address_id` are configured, `pulumi up` will raise an error when one or some of these `ip_configuration` blocks are removed. because the `public_ip_address_id` is still used by the `firewall` resource until the `firewall` resource is updated. and the destruction of `azure.network.PublicIp` happens before the update of firewall by default. to destroy of `azure.network.PublicIp` will cause the error. The workaround is to set `create_before_destroy=true` to the `azure.network.PublicIp` resource `lifecycle` block. See more detail: destroying.md#create-before-destroy
    /// 
    /// > **NOTE** The Public IP must have a `Static` allocation and `Standard` SKU.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Option<String>,
    /// Reference to the subnet associated with the IP Configuration. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE** The Subnet used for the Firewall must have the name `AzureFirewallSubnet` and the subnet mask must be at least a `/26`.
    /// 
    /// > **NOTE** At least one and only one `ip_configuration` block may contain a `subnet_id`.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
}
