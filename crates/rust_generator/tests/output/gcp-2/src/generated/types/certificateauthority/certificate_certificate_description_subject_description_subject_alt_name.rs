#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateCertificateDescriptionSubjectDescriptionSubjectAltName {
    /// (Output)
    /// Contains additional subject alternative name values.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "customSans")]
    pub r#custom_sans: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectDescriptionSubjectAltNameCustomSan>>>,
    /// Contains only valid, fully-qualified host names.
    #[builder(into, default)]
    #[serde(rename = "dnsNames")]
    pub r#dns_names: Box<Option<Vec<String>>>,
    /// Contains only valid RFC 2822 E-mail addresses.
    #[builder(into, default)]
    #[serde(rename = "emailAddresses")]
    pub r#email_addresses: Box<Option<Vec<String>>>,
    /// Contains only valid 32-bit IPv4 addresses or RFC 4291 IPv6 addresses.
    #[builder(into, default)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Box<Option<Vec<String>>>,
    /// Contains only valid RFC 3986 URIs.
    #[builder(into, default)]
    #[serde(rename = "uris")]
    pub r#uris: Box<Option<Vec<String>>>,
}
