#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderFeaturesKeyVault {
    /// When enabled soft-deleted `azure.keyvault.KeyVault` resources will be permanently deleted (e.g purged), when destroyed
    #[builder(into)]
    #[serde(rename = "purgeSoftDeleteOnDestroy")]
    pub r#purge_soft_delete_on_destroy: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.Certificate` resources will be permanently deleted (e.g purged), when destroyed
    #[builder(into)]
    #[serde(rename = "purgeSoftDeletedCertificatesOnDestroy")]
    pub r#purge_soft_deleted_certificates_on_destroy: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.ManagedHardwareSecurityModuleKey` resources will be permanently deleted (e.g purged), when destroyed
    #[builder(into)]
    #[serde(rename = "purgeSoftDeletedHardwareSecurityModuleKeysOnDestroy")]
    pub r#purge_soft_deleted_hardware_security_module_keys_on_destroy: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.ManagedHardwareSecurityModule` resources will be permanently deleted (e.g purged), when destroyed
    #[builder(into)]
    #[serde(rename = "purgeSoftDeletedHardwareSecurityModulesOnDestroy")]
    pub r#purge_soft_deleted_hardware_security_modules_on_destroy: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.Key` resources will be permanently deleted (e.g purged), when destroyed
    #[builder(into)]
    #[serde(rename = "purgeSoftDeletedKeysOnDestroy")]
    pub r#purge_soft_deleted_keys_on_destroy: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.Secret` resources will be permanently deleted (e.g purged), when destroyed
    #[builder(into)]
    #[serde(rename = "purgeSoftDeletedSecretsOnDestroy")]
    pub r#purge_soft_deleted_secrets_on_destroy: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.Certificate` resources will be restored, instead of creating new ones
    #[builder(into)]
    #[serde(rename = "recoverSoftDeletedCertificates")]
    pub r#recover_soft_deleted_certificates: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.ManagedHardwareSecurityModuleKey` resources will be restored, instead of creating new ones
    #[builder(into)]
    #[serde(rename = "recoverSoftDeletedHardwareSecurityModuleKeys")]
    pub r#recover_soft_deleted_hardware_security_module_keys: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.KeyVault` resources will be restored, instead of creating new ones
    #[builder(into)]
    #[serde(rename = "recoverSoftDeletedKeyVaults")]
    pub r#recover_soft_deleted_key_vaults: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.Key` resources will be restored, instead of creating new ones
    #[builder(into)]
    #[serde(rename = "recoverSoftDeletedKeys")]
    pub r#recover_soft_deleted_keys: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.Secret` resources will be restored, instead of creating new ones
    #[builder(into)]
    #[serde(rename = "recoverSoftDeletedSecrets")]
    pub r#recover_soft_deleted_secrets: Option<bool>,
}
