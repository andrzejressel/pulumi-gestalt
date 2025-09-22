#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterKeyManagementService {
    /// Identifier of Azure Key Vault key. See [key identifier format](https://learn.microsoft.com/en-us/azure/key-vault/general/about-keys-secrets-certificates#vault-name-and-object-name) for more details.
    #[builder(into)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: String,
    /// Network access of the key vault Network access of key vault. The possible values are `Public` and `Private`. `Public` means the key vault allows public access from all networks. `Private` means the key vault disables public access and enables private link. Defaults to `Public`.
    #[builder(into)]
    #[serde(rename = "keyVaultNetworkAccess")]
    pub r#key_vault_network_access: Option<String>,
}
