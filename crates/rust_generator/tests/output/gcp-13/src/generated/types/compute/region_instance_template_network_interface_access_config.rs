#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionInstanceTemplateNetworkInterfaceAccessConfig {
    /// The IP address that will be 1:1 mapped to the instance's
    /// network ip. If not given, one will be generated.
    #[builder(into)]
    #[serde(rename = "natIp")]
    pub r#nat_ip: Option<String>,
    /// The service-level to be provided for IPv6 traffic when the
    /// subnet has an external subnet. Only PREMIUM and STANDARD tier is valid for IPv6.
    #[builder(into)]
    #[serde(rename = "networkTier")]
    pub r#network_tier: Option<String>,
    /// The DNS domain name for the public PTR record.The DNS domain name for the public PTR record.
    #[builder(into)]
    #[serde(rename = "publicPtrDomainName")]
    pub r#public_ptr_domain_name: Option<String>,
}
