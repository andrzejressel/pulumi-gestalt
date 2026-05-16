#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicatedVmManagedDiskTargetDiskEncryptionKeyEncryptionKey {
    /// The URL to the Key Vault Key used as the Key Encryption Key that the Managed Disk will be associated with. This can be found as `id` on the `azure.keyvault.Key` resource. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "keyUrl")]
    pub r#key_url: String,
    /// The ID of the Key Vault. This can be found as `id` on the `azure.keyvault.KeyVault` resource. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "vaultId")]
    pub r#vault_id: String,
}
