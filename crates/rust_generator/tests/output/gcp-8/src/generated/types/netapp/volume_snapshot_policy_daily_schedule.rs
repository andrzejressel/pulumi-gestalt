#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VolumeSnapshotPolicyDailySchedule {
    /// Set the hour to create the snapshot (0-23), defaults to midnight (0).
    #[builder(into)]
    #[serde(rename = "hour")]
    pub r#hour: Option<i32>,
    /// Set the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0).
    #[builder(into)]
    #[serde(rename = "minute")]
    pub r#minute: Option<i32>,
    /// The maximum number of snapshots to keep for the daily schedule.
    #[builder(into)]
    #[serde(rename = "snapshotsToKeep")]
    pub r#snapshots_to_keep: i32,
}
