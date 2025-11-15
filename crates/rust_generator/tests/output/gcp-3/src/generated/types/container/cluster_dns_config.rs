#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterDnsConfig {
    /// This will enable Cloud DNS additive VPC scope. Must provide a domain name that is unique within the VPC. For this to work `cluster_dns = "CLOUD_DNS"` and `cluster_dns_scope = "CLUSTER_SCOPE"` must both be set as well.
    #[builder(into)]
    #[serde(rename = "additiveVpcScopeDnsDomain")]
    pub r#additive_vpc_scope_dns_domain: Option<String>,
    /// Which in-cluster DNS provider should be used. `PROVIDER_UNSPECIFIED` (default) or `PLATFORM_DEFAULT` or `CLOUD_DNS`.
    #[builder(into)]
    #[serde(rename = "clusterDns")]
    pub r#cluster_dns: Option<String>,
    /// The suffix used for all cluster service records.
    #[builder(into)]
    #[serde(rename = "clusterDnsDomain")]
    pub r#cluster_dns_domain: Option<String>,
    /// The scope of access to cluster DNS records. `DNS_SCOPE_UNSPECIFIED` (default) or `CLUSTER_SCOPE` or `VPC_SCOPE`.
    #[builder(into)]
    #[serde(rename = "clusterDnsScope")]
    pub r#cluster_dns_scope: Option<String>,
}
