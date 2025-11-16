#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupPlanBackupSchedule {
    /// A standard cron string that defines a repeating schedule for
    /// creating Backups via this BackupPlan.
    /// This is mutually exclusive with the rpoConfig field since at most one
    /// schedule can be defined for a BackupPlan.
    /// If this is defined, then backupRetainDays must also be defined.
    #[builder(into)]
    #[serde(rename = "cronSchedule")]
    pub r#cron_schedule: Option<String>,
    /// This flag denotes whether automatic Backup creation is paused for this BackupPlan.
    #[builder(into)]
    #[serde(rename = "paused")]
    pub r#paused: Option<bool>,
    /// Defines the RPO schedule configuration for this BackupPlan. This is mutually
    /// exclusive with the cronSchedule field since at most one schedule can be defined
    /// for a BackupPLan. If this is defined, then backupRetainDays must also be defined.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rpoConfig")]
    pub r#rpo_config: Option<Box<super::super::types::gkebackup::BackupPlanBackupScheduleRpoConfig>>,
}
