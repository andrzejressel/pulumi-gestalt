#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationBackupRetentionPolicyWeeklySchedule {
    /// A `retention_duration` block as defined below.
    #[builder(into)]
    #[serde(rename = "retentionDuration")]
    pub r#retention_duration: Option<Box<super::super::types::automanage::ConfigurationBackupRetentionPolicyWeeklyScheduleRetentionDuration>>,
    /// The retention times of the backup policy.
    #[builder(into)]
    #[serde(rename = "retentionTimes")]
    pub r#retention_times: Option<Vec<String>>,
}
