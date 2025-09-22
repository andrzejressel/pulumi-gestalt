#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceSecureLdap {
    /// The expiry time of the certificate.
    #[builder(into)]
    #[serde(rename = "certificateExpiry")]
    pub r#certificate_expiry: Option<String>,
    /// The thumbprint of the certificate.
    #[builder(into)]
    #[serde(rename = "certificateThumbprint")]
    pub r#certificate_thumbprint: Option<String>,
    /// Whether to enable secure LDAP for the managed domain. For more information, please see [official documentation on enabling LDAPS](https://docs.microsoft.com/azure/active-directory-domain-services/tutorial-configure-ldaps), paying particular attention to the section on network security to avoid unnecessarily exposing your service to Internet-borne bruteforce attacks.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Whether to enable external access to LDAPS over the Internet. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "externalAccessEnabled")]
    pub r#external_access_enabled: Option<bool>,
    /// The certificate/private key to use for LDAPS, as a base64-encoded TripleDES-SHA1 encrypted PKCS#12 bundle (PFX file).
    #[builder(into)]
    #[serde(rename = "pfxCertificate")]
    pub r#pfx_certificate: String,
    /// The password to use for decrypting the PKCS#12 bundle (PFX file).
    #[builder(into)]
    #[serde(rename = "pfxCertificatePassword")]
    pub r#pfx_certificate_password: String,
    /// The public certificate.
    #[builder(into)]
    #[serde(rename = "publicCertificate")]
    pub r#public_certificate: Option<String>,
}
