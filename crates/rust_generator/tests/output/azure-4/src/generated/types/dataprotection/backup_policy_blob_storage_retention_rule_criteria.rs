#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackupPolicyBlobStorageRetentionRuleCriteria {
    /// Possible values are `AllBackup`, `FirstOfDay`, `FirstOfWeek`, `FirstOfMonth` and `FirstOfYear`. These values mean the first successful backup of the day/week/month/year. Changing this forces a new Backup Policy Blob Storage to be created.
    #[builder(into)]
    #[serde(rename = "absoluteCriteria")]
    pub r#absolute_criteria: Option<String>,
    /// Must be between `0` and `28`. `0` for last day within the month. Changing this forces a new Backup Policy Blob Storage to be created.
    #[builder(into)]
    #[serde(rename = "daysOfMonths")]
    pub r#days_of_months: Option<Vec<i32>>,
    /// Possible values are `Monday`, `Tuesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`. Changing this forces a new Backup Policy Blob Storage to be created.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Option<Vec<String>>,
    /// Possible values are `January`, `February`, `March`, `April`, `May`, `June`, `July`, `August`, `September`, `October`, `November` and `December`. Changing this forces a new Backup Policy Blob Storage to be created.
    #[builder(into)]
    #[serde(rename = "monthsOfYears")]
    pub r#months_of_years: Option<Vec<String>>,
    /// Specifies a list of backup times for backup in the `RFC3339` format. Changing this forces a new Backup Policy Blob Storage to be created.
    #[builder(into)]
    #[serde(rename = "scheduledBackupTimes")]
    pub r#scheduled_backup_times: Option<Vec<String>>,
    /// Possible values are `First`, `Second`, `Third`, `Fourth` and `Last`. Changing this forces a new Backup Policy Blob Storage to be created.
    #[builder(into)]
    #[serde(rename = "weeksOfMonths")]
    pub r#weeks_of_months: Option<Vec<String>>,
}
