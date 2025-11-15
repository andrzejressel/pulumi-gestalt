#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineAutoBackupManualSchedule {
    /// A list of days on which backup can take place. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`
    /// 
    /// > **NOTE:** `days_of_week` can only be specified when `manual_schedule` is set to `Weekly`
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Option<Vec<String>>,
    /// Frequency of full backups. Valid values include `Daily` or `Weekly`.
    #[builder(into)]
    #[serde(rename = "fullBackupFrequency")]
    pub r#full_backup_frequency: String,
    /// Start hour of a given day during which full backups can take place. Valid values are from `0` to `23`.
    #[builder(into)]
    #[serde(rename = "fullBackupStartHour")]
    pub r#full_backup_start_hour: i32,
    /// Duration of the time window of a given day during which full backups can take place, in hours. Valid values are between `1` and `23`.
    #[builder(into)]
    #[serde(rename = "fullBackupWindowInHours")]
    pub r#full_backup_window_in_hours: i32,
    /// Frequency of log backups, in minutes. Valid values are from `5` to `60`.
    #[builder(into)]
    #[serde(rename = "logBackupFrequencyInMinutes")]
    pub r#log_backup_frequency_in_minutes: i32,
}
