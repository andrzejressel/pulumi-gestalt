#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkspaceEncryption {
    /// The Key Vault URI to access the encryption key.
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: String,
    /// The ID of the keyVault where the customer owned encryption key is present.
    #[builder(into)]
    #[serde(rename = "keyVaultId")]
    pub r#key_vault_id: String,
    /// The Key Vault URI to access the encryption key.
    /// 
    /// > **Note:** `user_assigned_identity_id` must set when`identity.type` is `UserAssigned` or service won't be able to find the assigned permissions.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Option<String>,
}
