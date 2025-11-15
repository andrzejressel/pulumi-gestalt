#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationBackupRetentionPolicy {
    /// A `daily_schedule` block as defined below.
    #[builder(into)]
    #[serde(rename = "dailySchedule")]
    pub r#daily_schedule: Option<Box<super::super::types::automanage::ConfigurationBackupRetentionPolicyDailySchedule>>,
    /// The retention policy type of the backup policy. Possible value is `LongTermRetentionPolicy`. Defaults to `LongTermRetentionPolicy`.
    #[builder(into)]
    #[serde(rename = "retentionPolicyType")]
    pub r#retention_policy_type: Option<String>,
    /// A `weekly_schedule` block as defined below.
    #[builder(into)]
    #[serde(rename = "weeklySchedule")]
    pub r#weekly_schedule: Option<Box<super::super::types::automanage::ConfigurationBackupRetentionPolicyWeeklySchedule>>,
}
