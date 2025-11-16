#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigSignInHashConfig {
    /// Different password hash algorithms used in Identity Toolkit.
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Option<String>,
    /// Memory cost for hash calculation. Used by scrypt and other similar password derivation algorithms. See https://tools.ietf.org/html/rfc7914 for explanation of field.
    #[builder(into)]
    #[serde(rename = "memoryCost")]
    pub r#memory_cost: Option<i32>,
    /// How many rounds for hash calculation. Used by scrypt and other similar password derivation algorithms.
    #[builder(into)]
    #[serde(rename = "rounds")]
    pub r#rounds: Option<i32>,
    /// Non-printable character to be inserted between the salt and plain text password in base64.
    #[builder(into)]
    #[serde(rename = "saltSeparator")]
    pub r#salt_separator: Option<String>,
    /// Signer key in base64.
    #[builder(into)]
    #[serde(rename = "signerKey")]
    pub r#signer_key: Option<String>,
}
