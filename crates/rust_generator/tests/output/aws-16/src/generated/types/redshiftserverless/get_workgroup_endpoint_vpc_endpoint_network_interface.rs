#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWorkgroupEndpointVpcEndpointNetworkInterface {
    /// The availability Zone.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: String,
    /// The unique identifier of the network interface.
    #[builder(into)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: String,
    /// The IPv4 address of the network interface within the subnet.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: String,
    /// The unique identifier of the subnet.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}
