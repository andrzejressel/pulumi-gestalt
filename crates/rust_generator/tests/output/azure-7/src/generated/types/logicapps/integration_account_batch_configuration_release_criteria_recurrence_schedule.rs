#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IntegrationAccountBatchConfigurationReleaseCriteriaRecurrenceSchedule {
    /// A list containing a single item, which specifies the Hour interval at which this recurrence should be triggered.
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: Option<Vec<i32>>,
    /// A list containing a single item which specifies the Minute interval at which this recurrence should be triggered.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Option<Vec<i32>>,
    /// A list of days of the month that the job should execute on.
    #[builder(into)]
    #[serde(rename = "monthDays")]
    pub r#month_days: Option<Vec<i32>>,
    /// A `monthly` block as documented below.
    #[builder(into)]
    #[serde(rename = "monthlies")]
    pub r#monthlies: Option<Vec<super::super::types::logicapps::IntegrationAccountBatchConfigurationReleaseCriteriaRecurrenceScheduleMonthly>>,
    /// A list of days of the week that the job should execute on. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` and `Saturday`.
    #[builder(into)]
    #[serde(rename = "weekDays")]
    pub r#week_days: Option<Vec<String>>,
}
