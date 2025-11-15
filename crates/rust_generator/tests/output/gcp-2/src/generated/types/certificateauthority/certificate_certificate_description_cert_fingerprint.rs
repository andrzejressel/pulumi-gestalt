#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateCertificateDescriptionCertFingerprint {
    /// (Output)
    /// The SHA 256 hash, encoded in hexadecimal, of the DER x509 certificate.
    #[builder(into)]
    #[serde(rename = "sha256Hash")]
    pub r#sha_256_hash: Option<String>,
}
