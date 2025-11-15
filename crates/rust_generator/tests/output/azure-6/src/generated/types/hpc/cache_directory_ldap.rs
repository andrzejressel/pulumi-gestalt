#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CacheDirectoryLdap {
    /// The base distinguished name (DN) for the LDAP domain.
    #[builder(into)]
    #[serde(rename = "baseDn")]
    pub r#base_dn: String,
    /// A `bind` block as defined above.
    #[builder(into)]
    #[serde(rename = "bind")]
    pub r#bind: Option<Box<super::super::types::hpc::CacheDirectoryLdapBind>>,
    /// The URI of the CA certificate to validate the LDAP secure connection.
    #[builder(into)]
    #[serde(rename = "certificateValidationUri")]
    pub r#certificate_validation_uri: Option<String>,
    /// Whether the certificate should be automatically downloaded. This can be set to `true` only when `certificate_validation_uri` is provided.
    #[builder(into)]
    #[serde(rename = "downloadCertificateAutomatically")]
    pub r#download_certificate_automatically: Option<bool>,
    /// Whether the LDAP connection should be encrypted?
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Option<bool>,
    /// The FQDN or IP address of the LDAP server.
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: String,
}
