#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterAutomatedBackupPolicyWeeklySchedule {
    /// The days of the week to perform a backup. At least one day of the week must be provided.
    /// Each value may be one of: `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Option<Vec<String>>,
    /// The times during the day to start a backup. At least one start time must be provided. The start times are assumed to be in UTC and to be an exact hour (e.g., 04:00:00).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "startTimes")]
    pub r#start_times: Vec<super::super::types::alloydb::ClusterAutomatedBackupPolicyWeeklyScheduleStartTime>,
}
