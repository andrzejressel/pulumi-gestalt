#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceStorageConfigStorageConfigS3ConfigEncryptionConfig {
    /// The type of encryption. Valid Values: `KMS`.
    #[builder(into)]
    #[serde(rename = "encryptionType")]
    pub r#encryption_type: String,
    /// The full ARN of the encryption key. Be sure to provide the full ARN of the encryption key, not just the ID.
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: String,
}
