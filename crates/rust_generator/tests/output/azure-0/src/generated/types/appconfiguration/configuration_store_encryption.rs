#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationStoreEncryption {
    /// Specifies the client id of the identity which will be used to access key vault.
    #[builder(into)]
    #[serde(rename = "identityClientId")]
    pub r#identity_client_id: Option<String>,
    /// Specifies the URI of the key vault key used to encrypt data.
    #[builder(into)]
    #[serde(rename = "keyVaultKeyIdentifier")]
    pub r#key_vault_key_identifier: Option<String>,
}
