#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsWebAppBackupSchedule {
    /// How often the backup should be executed (e.g. for weekly backup, this should be set to `7` and `frequency_unit` should be set to `Day`).
    /// 
    /// > **NOTE:** Not all intervals are supported on all Windows Web App SKUs. Please refer to the official documentation for appropriate values.
    #[builder(into)]
    #[serde(rename = "frequencyInterval")]
    pub r#frequency_interval: i32,
    /// The unit of time for how often the backup should take place. Possible values include: `Day`, `Hour`
    #[builder(into)]
    #[serde(rename = "frequencyUnit")]
    pub r#frequency_unit: String,
    /// Should the service keep at least one backup, regardless of age of backup. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "keepAtLeastOneBackup")]
    pub r#keep_at_least_one_backup: Option<bool>,
    /// The time the backup was last attempted.
    #[builder(into)]
    #[serde(rename = "lastExecutionTime")]
    pub r#last_execution_time: Option<String>,
    /// After how many days backups should be deleted. Defaults to `30`.
    #[builder(into)]
    #[serde(rename = "retentionPeriodDays")]
    pub r#retention_period_days: Option<i32>,
    /// When the schedule should start working in RFC-3339 format.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
}
