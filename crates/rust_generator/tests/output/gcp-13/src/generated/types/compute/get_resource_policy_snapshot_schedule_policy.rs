#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetResourcePolicySnapshotSchedulePolicy {
    /// Retention policy applied to snapshots created by this resource policy.
    #[builder(into)]
    #[serde(rename = "retentionPolicies")]
    pub r#retention_policies: Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicyRetentionPolicy>,
    /// Contains one of an 'hourlySchedule', 'dailySchedule', or 'weeklySchedule'.
    #[builder(into)]
    #[serde(rename = "schedules")]
    pub r#schedules: Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicySchedule>,
    /// Properties with which the snapshots are created, such as labels.
    #[builder(into)]
    #[serde(rename = "snapshotProperties")]
    pub r#snapshot_properties: Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicySnapshotProperty>,
}
