#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainAssociationSubDomain {
    /// Branch name setting for the subdomain.
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: String,
    /// DNS record for the subdomain in a space-prefixed and space-delimited format (` CNAME <target>`).
    #[builder(into)]
    #[serde(rename = "dnsRecord")]
    pub r#dns_record: Option<String>,
    /// Prefix setting for the subdomain.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: String,
    /// Verified status of the subdomain.
    #[builder(into)]
    #[serde(rename = "verified")]
    pub r#verified: Option<bool>,
}
