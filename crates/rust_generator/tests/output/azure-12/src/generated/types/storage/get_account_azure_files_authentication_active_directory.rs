#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAccountAzureFilesAuthenticationActiveDirectory {
    /// The domain GUID.
    #[builder(into)]
    #[serde(rename = "domainGuid")]
    pub r#domain_guid: String,
    /// The primary domain that the AD DNS server is authoritative for.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: String,
    /// The domain security identifier.
    #[builder(into)]
    #[serde(rename = "domainSid")]
    pub r#domain_sid: String,
    /// The name of the Active Directory forest.
    #[builder(into)]
    #[serde(rename = "forestName")]
    pub r#forest_name: String,
    /// The NetBIOS domain name.
    #[builder(into)]
    #[serde(rename = "netbiosDomainName")]
    pub r#netbios_domain_name: String,
    /// The security identifier for Azure Storage.
    #[builder(into)]
    #[serde(rename = "storageSid")]
    pub r#storage_sid: String,
}
