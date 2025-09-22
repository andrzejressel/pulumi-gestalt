#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZeroTrustLocalFallbackDomainDomain {
    /// A description of the fallback domain, displayed in the client UI.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// A list of IP addresses to handle domain resolution.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Option<Vec<String>>,
    /// The domain suffix to match when resolving locally.
    #[builder(into)]
    #[serde(rename = "suffix")]
    pub r#suffix: Option<String>,
}
