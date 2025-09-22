#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityScanConfigSchedule {
    /// The duration of time between executions in days
    #[builder(into)]
    #[serde(rename = "intervalDurationDays")]
    pub r#interval_duration_days: i32,
    /// A timestamp indicates when the next run will be scheduled. The value is refreshed
    /// by the server after each run. If unspecified, it will default to current server time,
    /// which means the scan will be scheduled to start immediately.
    #[builder(into)]
    #[serde(rename = "scheduleTime")]
    pub r#schedule_time: Option<String>,
}
