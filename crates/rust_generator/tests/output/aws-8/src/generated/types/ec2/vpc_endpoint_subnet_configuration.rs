#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpcEndpointSubnetConfiguration {
    /// The IPv4 address to assign to the endpoint network interface in the subnet. You must provide an IPv4 address if the VPC endpoint supports IPv4.
    #[builder(into)]
    #[serde(rename = "ipv4")]
    pub r#ipv_4: Option<String>,
    /// The IPv6 address to assign to the endpoint network interface in the subnet. You must provide an IPv6 address if the VPC endpoint supports IPv6.
    #[builder(into)]
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Option<String>,
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
}
