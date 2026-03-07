#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderFeaturesRecoveryService {
    #[builder(into)]
    #[serde(rename = "purgeProtectedItemsFromVaultOnDestroy")]
    pub r#purge_protected_items_from_vault_on_destroy: Option<bool>,
    #[builder(into)]
    #[serde(rename = "vmBackupStopProtectionAndRetainDataOnDestroy")]
    pub r#vm_backup_stop_protection_and_retain_data_on_destroy: Option<bool>,
}
