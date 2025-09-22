#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLaunchTemplateNetworkInterface {
    #[builder(into)]
    #[serde(rename = "associateCarrierIpAddress")]
    pub r#associate_carrier_ip_address: String,
    #[builder(into)]
    #[serde(rename = "associatePublicIpAddress")]
    pub r#associate_public_ip_address: Option<bool>,
    #[builder(into)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Option<bool>,
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    #[builder(into)]
    #[serde(rename = "deviceIndex")]
    pub r#device_index: i32,
    #[builder(into)]
    #[serde(rename = "interfaceType")]
    pub r#interface_type: String,
    #[builder(into)]
    #[serde(rename = "ipv4AddressCount")]
    pub r#ipv_4_address_count: i32,
    #[builder(into)]
    #[serde(rename = "ipv4Addresses")]
    pub r#ipv_4_addresses: Vec<String>,
    #[builder(into)]
    #[serde(rename = "ipv4PrefixCount")]
    pub r#ipv_4_prefix_count: i32,
    #[builder(into)]
    #[serde(rename = "ipv4Prefixes")]
    pub r#ipv_4_prefixes: Vec<String>,
    #[builder(into)]
    #[serde(rename = "ipv6AddressCount")]
    pub r#ipv_6_address_count: i32,
    #[builder(into)]
    #[serde(rename = "ipv6Addresses")]
    pub r#ipv_6_addresses: Vec<String>,
    #[builder(into)]
    #[serde(rename = "ipv6PrefixCount")]
    pub r#ipv_6_prefix_count: i32,
    #[builder(into)]
    #[serde(rename = "ipv6Prefixes")]
    pub r#ipv_6_prefixes: Vec<String>,
    #[builder(into)]
    #[serde(rename = "networkCardIndex")]
    pub r#network_card_index: i32,
    #[builder(into)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: String,
    #[builder(into)]
    #[serde(rename = "primaryIpv6")]
    pub r#primary_ipv_6: String,
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: String,
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Vec<String>,
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}
