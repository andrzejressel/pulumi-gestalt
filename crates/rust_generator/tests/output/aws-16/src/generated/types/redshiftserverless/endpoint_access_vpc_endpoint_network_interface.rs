#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointAccessVpcEndpointNetworkInterface {
    /// The availability Zone.
    #[builder(into, default)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<Option<String>>,
    /// The unique identifier of the network interface.
    #[builder(into, default)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: Box<Option<String>>,
    /// The IPv4 address of the network interface within the subnet.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// The unique identifier of the subnet.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
}
