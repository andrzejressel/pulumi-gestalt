#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HciNetworkInterfaceIpConfiguration {
    /// The IPv4 address of the gateway for the Network Interface.
    #[builder(into)]
    #[serde(rename = "gateway")]
    pub r#gateway: Option<String>,
    /// The prefix length for the address of the Network Interface.
    #[builder(into)]
    #[serde(rename = "prefixLength")]
    pub r#prefix_length: Option<String>,
    /// The IPv4 address of the IP configuration. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Option<String>,
    /// The resource ID of the Stack HCI Logical Network bound to the IP configuration. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}
