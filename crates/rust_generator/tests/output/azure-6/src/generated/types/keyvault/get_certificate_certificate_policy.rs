#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCertificateCertificatePolicy {
    /// A `issuer_parameters` block as defined below.
    #[builder(into)]
    #[serde(rename = "issuerParameters")]
    pub r#issuer_parameters: Vec<super::super::types::keyvault::GetCertificateCertificatePolicyIssuerParameter>,
    /// A `key_properties` block as defined below.
    #[builder(into)]
    #[serde(rename = "keyProperties")]
    pub r#key_properties: Vec<super::super::types::keyvault::GetCertificateCertificatePolicyKeyProperty>,
    /// A `lifetime_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "lifetimeActions")]
    pub r#lifetime_actions: Vec<super::super::types::keyvault::GetCertificateCertificatePolicyLifetimeAction>,
    /// A `secret_properties` block as defined below.
    #[builder(into)]
    #[serde(rename = "secretProperties")]
    pub r#secret_properties: Vec<super::super::types::keyvault::GetCertificateCertificatePolicySecretProperty>,
    /// An `x509_certificate_properties` block as defined below.
    #[builder(into)]
    #[serde(rename = "x509CertificateProperties")]
    pub r#x_509_certificate_properties: Vec<super::super::types::keyvault::GetCertificateCertificatePolicyX509CertificateProperty>,
}
