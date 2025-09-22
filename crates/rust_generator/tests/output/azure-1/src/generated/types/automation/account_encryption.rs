#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountEncryption {
    #[builder(into)]
    #[serde(rename = "keySource")]
    pub r#key_source: Option<String>,
    /// The ID of the Key Vault Key which should be used to Encrypt the data in this Automation Account.
    #[builder(into)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: String,
    /// The User Assigned Managed Identity ID to be used for accessing the Customer Managed Key for encryption.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Option<String>,
}
