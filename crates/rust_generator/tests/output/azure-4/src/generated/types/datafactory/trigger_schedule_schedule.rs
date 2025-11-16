#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerScheduleSchedule {
    /// Day(s) of the month on which the trigger is scheduled. This value can be specified with a monthly frequency only.
    #[builder(into)]
    #[serde(rename = "daysOfMonths")]
    pub r#days_of_months: Option<Vec<i32>>,
    /// Days of the week on which the trigger is scheduled. This value can be specified only with a weekly frequency.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Option<Vec<String>>,
    /// Hours of the day on which the trigger is scheduled.
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: Option<Vec<i32>>,
    /// Minutes of the hour on which the trigger is scheduled.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Option<Vec<i32>>,
    /// A `monthly` block as documented below, which specifies the days of the month on which the trigger is scheduled. The value can be specified only with a monthly frequency.
    #[builder(into)]
    #[serde(rename = "monthlies")]
    pub r#monthlies: Option<Vec<super::super::types::datafactory::TriggerScheduleScheduleMonthly>>,
}
