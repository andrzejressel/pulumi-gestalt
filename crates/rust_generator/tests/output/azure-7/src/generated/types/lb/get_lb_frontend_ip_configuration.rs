#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLbFrontendIpConfiguration {
    /// The id of the Frontend IP Configuration.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Specifies the name of the Load Balancer.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Private IP Address to assign to the Load Balancer.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: String,
    /// The allocation method for the Private IP Address used by this Load Balancer.
    #[builder(into)]
    #[serde(rename = "privateIpAddressAllocation")]
    pub r#private_ip_address_allocation: String,
    /// The Private IP Address Version, either `IPv4` or `IPv6`.
    #[builder(into)]
    #[serde(rename = "privateIpAddressVersion")]
    pub r#private_ip_address_version: String,
    /// The ID of a  Public IP Address which is associated with this Load Balancer.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: String,
    /// The ID of the Subnet which is associated with the IP Configuration.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
    /// A list of Availability Zones which the Load Balancer's IP Addresses should be created in.
    #[builder(into)]
    #[serde(rename = "zones")]
    pub r#zones: Vec<String>,
}
