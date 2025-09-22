#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAuthorityConfigX509ConfigKeyUsageBaseKeyUsage {
    /// The key may be used to sign certificates.
    #[builder(into)]
    #[serde(rename = "certSign")]
    pub r#cert_sign: bool,
    /// The key may be used for cryptographic commitments. Note that this may also be referred to as "non-repudiation".
    #[builder(into)]
    #[serde(rename = "contentCommitment")]
    pub r#content_commitment: bool,
    /// The key may be used sign certificate revocation lists.
    #[builder(into)]
    #[serde(rename = "crlSign")]
    pub r#crl_sign: bool,
    /// The key may be used to encipher data.
    #[builder(into)]
    #[serde(rename = "dataEncipherment")]
    pub r#data_encipherment: bool,
    /// The key may be used to decipher only.
    #[builder(into)]
    #[serde(rename = "decipherOnly")]
    pub r#decipher_only: bool,
    /// The key may be used for digital signatures.
    #[builder(into)]
    #[serde(rename = "digitalSignature")]
    pub r#digital_signature: bool,
    /// The key may be used to encipher only.
    #[builder(into)]
    #[serde(rename = "encipherOnly")]
    pub r#encipher_only: bool,
    /// The key may be used in a key agreement protocol.
    #[builder(into)]
    #[serde(rename = "keyAgreement")]
    pub r#key_agreement: bool,
    /// The key may be used to encipher other keys.
    #[builder(into)]
    #[serde(rename = "keyEncipherment")]
    pub r#key_encipherment: bool,
}
