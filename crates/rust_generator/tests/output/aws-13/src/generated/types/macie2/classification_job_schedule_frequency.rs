#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClassificationJobScheduleFrequency {
    /// Specifies a daily recurrence pattern for running the job.
    #[builder(into)]
    #[serde(rename = "dailySchedule")]
    pub r#daily_schedule: Option<bool>,
    /// Specifies a monthly recurrence pattern for running the job.
    #[builder(into)]
    #[serde(rename = "monthlySchedule")]
    pub r#monthly_schedule: Option<i32>,
    /// Specifies a weekly recurrence pattern for running the job.
    #[builder(into)]
    #[serde(rename = "weeklySchedule")]
    pub r#weekly_schedule: Option<String>,
}
