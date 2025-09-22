#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterDnsConfig {
    /// Enable additive VPC scope DNS in a GKE cluster.
    #[builder(into)]
    #[serde(rename = "additiveVpcScopeDnsDomain")]
    pub r#additive_vpc_scope_dns_domain: String,
    /// Which in-cluster DNS provider should be used.
    #[builder(into)]
    #[serde(rename = "clusterDns")]
    pub r#cluster_dns: String,
    /// The suffix used for all cluster service records.
    #[builder(into)]
    #[serde(rename = "clusterDnsDomain")]
    pub r#cluster_dns_domain: String,
    /// The scope of access to cluster DNS records.
    #[builder(into)]
    #[serde(rename = "clusterDnsScope")]
    pub r#cluster_dns_scope: String,
}
