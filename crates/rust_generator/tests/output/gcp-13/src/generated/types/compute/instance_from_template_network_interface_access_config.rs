#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceFromTemplateNetworkInterfaceAccessConfig {
    /// The IP address that is be 1:1 mapped to the instance's network ip.
    #[builder(into)]
    #[serde(rename = "natIp")]
    pub r#nat_ip: Option<String>,
    /// The networking tier used for configuring this instance. One of PREMIUM or STANDARD.
    #[builder(into)]
    #[serde(rename = "networkTier")]
    pub r#network_tier: Option<String>,
    /// The DNS domain name for the public PTR record.
    #[builder(into)]
    #[serde(rename = "publicPtrDomainName")]
    pub r#public_ptr_domain_name: Option<String>,
    /// A full or partial URL to a security policy to add to this instance. If this field is set to an empty string it will remove the associated security policy.
    #[builder(into)]
    #[serde(rename = "securityPolicy")]
    pub r#security_policy: Option<String>,
}
