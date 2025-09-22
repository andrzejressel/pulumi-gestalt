#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinkServiceNatIpConfiguration {
    /// Specifies the name which should be used for the NAT IP Configuration. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Is this is the Primary IP Configuration? Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: bool,
    /// Specifies a Private Static IP Address for this IP Configuration.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Option<String>,
    /// The version of the IP Protocol which should be used. At this time the only supported value is `IPv4`. Defaults to `IPv4`.
    #[builder(into)]
    #[serde(rename = "privateIpAddressVersion")]
    pub r#private_ip_address_version: Option<String>,
    /// Specifies the ID of the Subnet which should be used for the Private Link Service.
    /// 
    /// > **NOTE:** Verify that the Subnet's `enforce_private_link_service_network_policies` attribute is set to `true`.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}
