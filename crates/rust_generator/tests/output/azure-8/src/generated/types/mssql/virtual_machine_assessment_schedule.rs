#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualMachineAssessmentSchedule {
    /// What day of the week the assessment will be run. Possible values are `Friday`, `Monday`, `Saturday`, `Sunday`, `Thursday`, `Tuesday` and `Wednesday`.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: String,
    /// How many months between assessment runs. Valid values are between `1` and `5`.
    /// 
    /// > **NOTE:** Either one of `weekly_interval` or `monthly_occurrence` must be specified.
    #[builder(into)]
    #[serde(rename = "monthlyOccurrence")]
    pub r#monthly_occurrence: Option<i32>,
    /// What time the assessment will be run. Must be in the format `HH:mm`.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: String,
    /// How many weeks between assessment runs. Valid values are between `1` and `6`.
    #[builder(into)]
    #[serde(rename = "weeklyInterval")]
    pub r#weekly_interval: Option<i32>,
}
