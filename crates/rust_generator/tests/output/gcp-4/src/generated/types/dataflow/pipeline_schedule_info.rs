#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineScheduleInfo {
    /// (Output)
    /// When the next Scheduler job is going to run.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "nextJobTime")]
    pub r#next_job_time: Option<String>,
    /// Unix-cron format of the schedule. This information is retrieved from the linked Cloud Scheduler.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Option<String>,
    /// Timezone ID. This matches the timezone IDs used by the Cloud Scheduler API. If empty, UTC time is assumed.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
}
