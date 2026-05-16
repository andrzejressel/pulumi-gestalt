#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBackupBackup {
    /// Id of the requesting object, Backup.
    #[builder(into)]
    #[serde(rename = "backupId")]
    pub r#backup_id: String,
    /// Name of the Backup Vault associated with Backup.
    #[builder(into)]
    #[serde(rename = "backupVaultId")]
    pub r#backup_vault_id: String,
    /// Name of the Data Source associated with Backup.
    #[builder(into)]
    #[serde(rename = "dataSourceId")]
    pub r#data_source_id: String,
    /// Location of the resource.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// Name of the resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
