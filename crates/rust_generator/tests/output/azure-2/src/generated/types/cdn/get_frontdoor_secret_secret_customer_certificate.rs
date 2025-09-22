#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFrontdoorSecretSecretCustomerCertificate {
    /// The key vault certificate expiration date.
    #[builder(into)]
    #[serde(rename = "expirationDate")]
    pub r#expiration_date: String,
    /// The key vault certificate ID.
    #[builder(into)]
    #[serde(rename = "keyVaultCertificateId")]
    pub r#key_vault_certificate_id: String,
    /// One or more `subject alternative names` contained within the key vault certificate.
    #[builder(into)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Vec<String>,
}
