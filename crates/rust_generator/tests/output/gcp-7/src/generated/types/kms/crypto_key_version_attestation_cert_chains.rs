#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CryptoKeyVersionAttestationCertChains {
    /// Cavium certificate chain corresponding to the attestation.
    #[builder(into)]
    #[serde(rename = "caviumCerts")]
    pub r#cavium_certs: Option<Vec<String>>,
    /// Google card certificate chain corresponding to the attestation.
    #[builder(into)]
    #[serde(rename = "googleCardCerts")]
    pub r#google_card_certs: Option<Vec<String>>,
    /// Google partition certificate chain corresponding to the attestation.
    #[builder(into)]
    #[serde(rename = "googlePartitionCerts")]
    pub r#google_partition_certs: Option<Vec<String>>,
}
