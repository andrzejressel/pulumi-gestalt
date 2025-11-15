#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KeyKeyAttributesKeyModesOfUse {
    /// Whether an AWS Payment Cryptography key can be used to decrypt data.
    #[builder(into)]
    #[serde(rename = "decrypt")]
    pub r#decrypt: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used to derive new keys.
    #[builder(into)]
    #[serde(rename = "deriveKey")]
    pub r#derive_key: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used to encrypt data.
    #[builder(into)]
    #[serde(rename = "encrypt")]
    pub r#encrypt: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used to generate and verify other card and PIN verification keys.
    #[builder(into)]
    #[serde(rename = "generate")]
    pub r#generate: Option<bool>,
    /// Whether an AWS Payment Cryptography key has no special restrictions other than the restrictions implied by KeyUsage.
    #[builder(into)]
    #[serde(rename = "noRestrictions")]
    pub r#no_restrictions: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used for signing.
    #[builder(into)]
    #[serde(rename = "sign")]
    pub r#sign: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used to unwrap other keys.
    #[builder(into)]
    #[serde(rename = "unwrap")]
    pub r#unwrap: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used to verify signatures.
    #[builder(into)]
    #[serde(rename = "verify")]
    pub r#verify: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used to wrap other keys.
    #[builder(into)]
    #[serde(rename = "wrap")]
    pub r#wrap: Option<bool>,
}
