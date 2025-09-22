#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsVirtualMachineScaleSetExtensionProtectedSettingsFromKeyVault {
    /// The URL to the Key Vault Secret which stores the protected settings.
    #[builder(into)]
    #[serde(rename = "secretUrl")]
    pub r#secret_url: String,
    /// The ID of the source Key Vault.
    #[builder(into)]
    #[serde(rename = "sourceVaultId")]
    pub r#source_vault_id: String,
}
