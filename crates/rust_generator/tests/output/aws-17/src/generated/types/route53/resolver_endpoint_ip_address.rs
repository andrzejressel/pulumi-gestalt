#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResolverEndpointIpAddress {
    /// IPv4 address in the subnet that you want to use for DNS queries.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Option<String>,
    #[builder(into)]
    #[serde(rename = "ipId")]
    pub r#ip_id: Option<String>,
    /// IPv6 address in the subnet that you want to use for DNS queries.
    #[builder(into)]
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Option<String>,
    /// ID of the subnet that contains the IP address.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}
