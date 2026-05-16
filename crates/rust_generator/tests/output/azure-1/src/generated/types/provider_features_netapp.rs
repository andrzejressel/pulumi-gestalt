#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderFeaturesNetapp {
    /// When enabled, backups will be deleted when the `azure.netapp.BackupVault` resource is destroyed
    #[builder(into)]
    #[serde(rename = "deleteBackupsOnBackupVaultDestroy")]
    pub r#delete_backups_on_backup_vault_destroy: Option<bool>,
    /// When enabled, the volume will not be destroyed, safeguarding from severe data loss
    #[builder(into)]
    #[serde(rename = "preventVolumeDestruction")]
    pub r#prevent_volume_destruction: Option<bool>,
}
