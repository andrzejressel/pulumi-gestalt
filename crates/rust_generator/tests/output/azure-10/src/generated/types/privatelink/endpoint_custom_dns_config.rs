#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointCustomDnsConfig {
    /// The fully qualified domain name to the `private_dns_zone`.
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Option<String>,
    /// A list of all IP Addresses that map to the `private_dns_zone` fqdn.
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Option<Vec<String>>,
}
