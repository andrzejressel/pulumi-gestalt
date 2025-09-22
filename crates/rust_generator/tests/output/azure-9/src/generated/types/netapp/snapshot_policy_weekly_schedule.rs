#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SnapshotPolicyWeeklySchedule {
    /// List of the week days using English names when the snapshots will be created.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Vec<String>,
    /// Hour of the day that the snapshots will be created, valid range is from 0 to 23.
    #[builder(into)]
    #[serde(rename = "hour")]
    pub r#hour: i32,
    /// Minute of the hour that the snapshots will be created, valid range is from 0 to 59.
    #[builder(into)]
    #[serde(rename = "minute")]
    pub r#minute: i32,
    /// How many hourly snapshots to keep, valid range is from 0 to 255.
    #[builder(into)]
    #[serde(rename = "snapshotsToKeep")]
    pub r#snapshots_to_keep: i32,
}
