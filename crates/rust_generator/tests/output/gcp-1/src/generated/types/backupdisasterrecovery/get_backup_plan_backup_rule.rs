#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBackupPlanBackupRule {
    /// Configures the duration for which backup data will be kept. The value should be greater than or equal to minimum enforced retention of the backup vault.
    #[builder(into)]
    #[serde(rename = "backupRetentionDays")]
    pub r#backup_retention_days: i32,
    /// The unique ID of this 'BackupRule'. The 'rule_id' is unique per 'BackupPlan'.
    #[builder(into)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: String,
    /// StandardSchedule defines a schedule that runs within the confines of a defined window of days.
    #[builder(into)]
    #[serde(rename = "standardSchedules")]
    pub r#standard_schedules: Vec<super::super::types::backupdisasterrecovery::GetBackupPlanBackupRuleStandardSchedule>,
}
