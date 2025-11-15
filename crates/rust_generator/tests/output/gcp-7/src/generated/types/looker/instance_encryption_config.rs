#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceEncryptionConfig {
    /// Name of the customer managed encryption key (CMEK) in KMS.
    #[builder(into)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Option<String>,
    /// (Output)
    /// Full name and version of the CMEK key currently in use to encrypt Looker data.
    #[builder(into)]
    #[serde(rename = "kmsKeyNameVersion")]
    pub r#kms_key_name_version: Option<String>,
    /// (Output)
    /// Status of the customer managed encryption key (CMEK) in KMS.
    #[builder(into)]
    #[serde(rename = "kmsKeyState")]
    pub r#kms_key_state: Option<String>,
}
