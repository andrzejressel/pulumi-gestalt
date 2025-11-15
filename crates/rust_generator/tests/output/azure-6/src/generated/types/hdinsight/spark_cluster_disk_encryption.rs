#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SparkClusterDiskEncryption {
    /// This is an algorithm identifier for encryption. Possible values are `RSA1_5`, `RSA-OAEP`, `RSA-OAEP-256`.
    #[builder(into)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: Option<String>,
    /// This is indicator to show whether resource disk encryption is enabled.
    #[builder(into)]
    #[serde(rename = "encryptionAtHostEnabled")]
    pub r#encryption_at_host_enabled: Option<bool>,
    /// The ID of the key vault key.
    #[builder(into)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: Option<String>,
    /// This is the resource ID of Managed Identity used to access the key vault.
    #[builder(into)]
    #[serde(rename = "keyVaultManagedIdentityId")]
    pub r#key_vault_managed_identity_id: Option<String>,
}
