#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PatchDeploymentRecurringScheduleMonthlyWeekDayOfMonth {
    /// A day of the week.
    /// Possible values are: `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: String,
    /// Represents the number of days before or after the given week day of month that the patch deployment is scheduled for.
    #[builder(into)]
    #[serde(rename = "dayOffset")]
    pub r#day_offset: Option<i32>,
    /// Week number in a month. 1-4 indicates the 1st to 4th week of the month. -1 indicates the last week of the month.
    #[builder(into)]
    #[serde(rename = "weekOrdinal")]
    pub r#week_ordinal: i32,
}
