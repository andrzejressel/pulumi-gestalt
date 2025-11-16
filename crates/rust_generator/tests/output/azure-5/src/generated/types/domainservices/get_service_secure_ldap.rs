#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceSecureLdap {
    #[builder(into)]
    #[serde(rename = "certificateExpiry")]
    pub r#certificate_expiry: String,
    #[builder(into)]
    #[serde(rename = "certificateThumbprint")]
    pub r#certificate_thumbprint: String,
    /// Whether secure LDAP is enabled for the managed domain.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Whether external access to LDAPS over the Internet, is enabled.
    #[builder(into)]
    #[serde(rename = "externalAccessEnabled")]
    pub r#external_access_enabled: bool,
    #[builder(into)]
    #[serde(rename = "publicCertificate")]
    pub r#public_certificate: String,
}
