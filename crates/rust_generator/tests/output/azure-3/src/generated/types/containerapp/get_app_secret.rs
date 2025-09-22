#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAppSecret {
    /// Resource ID for the User Assigned Managed identity to use when pulling from the Container Registry.
    #[builder(into)]
    #[serde(rename = "identity")]
    pub r#identity: String,
    /// The ID of a Key Vault secret.
    #[builder(into)]
    #[serde(rename = "keyVaultSecretId")]
    pub r#key_vault_secret_id: String,
    /// The name of the Container App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The HTTP Header value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
