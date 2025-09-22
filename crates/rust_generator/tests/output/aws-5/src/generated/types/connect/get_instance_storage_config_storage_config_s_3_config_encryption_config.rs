#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceStorageConfigStorageConfigS3ConfigEncryptionConfig {
    /// The type of encryption. Valid Values: `KMS`.
    #[builder(into)]
    #[serde(rename = "encryptionType")]
    pub r#encryption_type: String,
    /// The full ARN of the encryption key. Be sure to provide the full ARN of the encryption key, not just the ID.
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: String,
}
