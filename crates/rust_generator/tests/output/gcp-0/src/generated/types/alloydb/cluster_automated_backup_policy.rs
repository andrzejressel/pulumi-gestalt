#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterAutomatedBackupPolicy {
    /// The length of the time window during which a backup can be taken. If a backup does not succeed within this time window, it will be canceled and considered failed.
    /// The backup window must be at least 5 minutes long. There is no upper bound on the window. If not set, it will default to 1 hour.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "backupWindow")]
    pub r#backup_window: Option<String>,
    /// Whether automated backups are enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// EncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Option<Box<super::super::types::alloydb::ClusterAutomatedBackupPolicyEncryptionConfig>>,
    /// Labels to apply to backups created using this configuration.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// The location where the backup will be stored. Currently, the only supported option is to store the backup in the same region as the cluster.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// Quantity-based Backup retention policy to retain recent backups. Conflicts with 'time_based_retention', both can't be set together.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "quantityBasedRetention")]
    pub r#quantity_based_retention: Option<Box<super::super::types::alloydb::ClusterAutomatedBackupPolicyQuantityBasedRetention>>,
    /// Time-based Backup retention policy. Conflicts with 'quantity_based_retention', both can't be set together.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "timeBasedRetention")]
    pub r#time_based_retention: Option<Box<super::super::types::alloydb::ClusterAutomatedBackupPolicyTimeBasedRetention>>,
    /// Weekly schedule for the Backup.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "weeklySchedule")]
    pub r#weekly_schedule: Option<Box<super::super::types::alloydb::ClusterAutomatedBackupPolicyWeeklySchedule>>,
}
