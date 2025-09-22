#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CryptoKeyVersionAttestation {
    /// The certificate chains needed to validate the attestation
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "certChains")]
    pub r#cert_chains: Option<Box<super::super::types::kms::CryptoKeyVersionAttestationCertChains>>,
    /// (Output)
    /// The attestation data provided by the HSM when the key operation was performed.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// ExternalProtectionLevelOptions stores a group of additional fields for configuring a CryptoKeyVersion that are specific to the EXTERNAL protection level and EXTERNAL_VPC protection levels.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "externalProtectionLevelOptions")]
    pub r#external_protection_level_options: Option<Box<super::super::types::kms::CryptoKeyVersionAttestationExternalProtectionLevelOptions>>,
    /// (Output)
    /// The format of the attestation data.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Option<String>,
}
