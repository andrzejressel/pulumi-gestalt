#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseInstanceSettingsBackupConfiguration {
    /// Backup retention settings. The configuration is detailed below.
    #[builder(into)]
    #[serde(rename = "backupRetentionSettings")]
    pub r#backup_retention_settings: Option<Box<super::super::types::sql::DatabaseInstanceSettingsBackupConfigurationBackupRetentionSettings>>,
    /// True if binary logging is enabled.
    /// Can only be used with MySQL.
    #[builder(into)]
    #[serde(rename = "binaryLogEnabled")]
    pub r#binary_log_enabled: Option<bool>,
    /// True if backup configuration is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The region where the backup will be stored
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// True if Point-in-time recovery is enabled. Will restart database if enabled after instance creation. Valid only for PostgreSQL and SQL Server instances.
    #[builder(into)]
    #[serde(rename = "pointInTimeRecoveryEnabled")]
    pub r#point_in_time_recovery_enabled: Option<bool>,
    /// `HH:MM` format time indicating when backup
    /// configuration starts.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    /// The number of days of transaction logs we retain for point in time restore, from 1-7. For PostgreSQL Enterprise Plus instances, the number of days of retained transaction logs can be set from 1 to 35.
    #[builder(into)]
    #[serde(rename = "transactionLogRetentionDays")]
    pub r#transaction_log_retention_days: Option<i32>,
}
