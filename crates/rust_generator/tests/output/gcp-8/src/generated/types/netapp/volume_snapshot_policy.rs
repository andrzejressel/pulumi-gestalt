#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VolumeSnapshotPolicy {
    /// Daily schedule policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dailySchedule")]
    pub r#daily_schedule: Option<Box<super::super::types::netapp::VolumeSnapshotPolicyDailySchedule>>,
    /// Enables automated snapshot creation according to defined schedule. Default is false.
    /// To disable automatic snapshot creation you have to remove the whole snapshot_policy block.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Hourly schedule policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hourlySchedule")]
    pub r#hourly_schedule: Option<Box<super::super::types::netapp::VolumeSnapshotPolicyHourlySchedule>>,
    /// Monthly schedule policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "monthlySchedule")]
    pub r#monthly_schedule: Option<Box<super::super::types::netapp::VolumeSnapshotPolicyMonthlySchedule>>,
    /// Weekly schedule policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "weeklySchedule")]
    pub r#weekly_schedule: Option<Box<super::super::types::netapp::VolumeSnapshotPolicyWeeklySchedule>>,
}
