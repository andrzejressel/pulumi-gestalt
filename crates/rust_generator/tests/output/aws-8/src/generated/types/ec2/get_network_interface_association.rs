#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkInterfaceAssociation {
    /// Allocation ID.
    #[builder(into)]
    #[serde(rename = "allocationId")]
    pub r#allocation_id: String,
    /// Association ID.
    #[builder(into)]
    #[serde(rename = "associationId")]
    pub r#association_id: String,
    /// Carrier IP address associated with the network interface. This attribute is only set when the network interface is in a subnet which is associated with a Wavelength Zone.
    #[builder(into)]
    #[serde(rename = "carrierIp")]
    pub r#carrier_ip: String,
    /// Customer-owned IP address.
    #[builder(into)]
    #[serde(rename = "customerOwnedIp")]
    pub r#customer_owned_ip: String,
    /// ID of the Elastic IP address owner.
    #[builder(into)]
    #[serde(rename = "ipOwnerId")]
    pub r#ip_owner_id: String,
    /// Public DNS name.
    #[builder(into)]
    #[serde(rename = "publicDnsName")]
    pub r#public_dns_name: String,
    /// Address of the Elastic IP address bound to the network interface.
    #[builder(into)]
    #[serde(rename = "publicIp")]
    pub r#public_ip: String,
}
