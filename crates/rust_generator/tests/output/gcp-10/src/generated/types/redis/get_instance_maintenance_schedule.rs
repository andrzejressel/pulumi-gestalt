#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceMaintenanceSchedule {
    /// Output only. The end time of any upcoming scheduled maintenance for this instance.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
    /// resolution and up to nine fractional digits.
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: String,
    /// Output only. The deadline that the maintenance schedule start time
    /// can not go beyond, including reschedule.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
    /// resolution and up to nine fractional digits.
    #[builder(into)]
    #[serde(rename = "scheduleDeadlineTime")]
    pub r#schedule_deadline_time: String,
    /// Output only. The start time of any upcoming scheduled maintenance for this instance.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
    /// resolution and up to nine fractional digits.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: String,
}
