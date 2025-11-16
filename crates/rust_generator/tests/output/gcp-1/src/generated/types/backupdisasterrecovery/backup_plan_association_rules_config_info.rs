#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupPlanAssociationRulesConfigInfo {
    /// (Output)
    /// google.rpc.Status object to store the last backup error
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "lastBackupErrors")]
    pub r#last_backup_errors: Option<Vec<super::super::types::backupdisasterrecovery::BackupPlanAssociationRulesConfigInfoLastBackupError>>,
    /// (Output)
    /// State of last backup taken.
    #[builder(into)]
    #[serde(rename = "lastBackupState")]
    pub r#last_backup_state: Option<String>,
    /// (Output)
    /// Backup Rule id fetched from backup plan.
    #[builder(into)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: Option<String>,
}
