#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationBackupRetentionPolicyDailyScheduleRetentionDuration {
    /// The count of the retention duration of the backup policy. Valid value inside `daily_schedule` is `7` to `9999` and inside `weekly_schedule` is `1` to `5163`.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Option<i32>,
    /// The duration type of the retention duration of the backup policy. Valid value inside `daily_schedule` is `Days` and inside `weekly_schedule` is `Weeks`. Defaults to `Days`.
    #[builder(into)]
    #[serde(rename = "durationType")]
    pub r#duration_type: Option<String>,
}
