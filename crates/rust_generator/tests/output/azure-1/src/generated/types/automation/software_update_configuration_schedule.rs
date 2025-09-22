#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SoftwareUpdateConfigurationSchedule {
    /// List of days of the month that the job should execute on. Must be between `1` and `31`. `-1` for last day of the month. Only valid when frequency is `Month`.
    #[builder(into)]
    #[serde(rename = "advancedMonthDays")]
    pub r#advanced_month_days: Option<Vec<i32>>,
    /// List of days of the week that the job should execute on. Only valid when frequency is `Week`. Possible values include `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday`, and `Sunday`.
    #[builder(into)]
    #[serde(rename = "advancedWeekDays")]
    pub r#advanced_week_days: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "creationTime")]
    pub r#creation_time: Option<String>,
    /// A description for this Schedule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The end time of the schedule.
    #[builder(into)]
    #[serde(rename = "expiryTime")]
    pub r#expiry_time: Option<String>,
    #[builder(into)]
    #[serde(rename = "expiryTimeOffsetMinutes")]
    pub r#expiry_time_offset_minutes: Option<f64>,
    /// The frequency of the schedule. - can be either `OneTime`, `Day`, `Hour`, `Week`, or `Month`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: String,
    /// The number of `frequency`s between runs. Only valid when frequency is `Day`, `Hour`, `Week`, or `Month`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Option<i32>,
    /// Whether the schedule is enabled. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "isEnabled")]
    pub r#is_enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "lastModifiedTime")]
    pub r#last_modified_time: Option<String>,
    /// List of `monthly_occurrence` blocks as defined below to specifies occurrences of days within a month. Only valid when frequency is `Month`. The `monthly_occurrence` block supports fields as defined below.
    #[builder(into)]
    #[serde(rename = "monthlyOccurrence")]
    pub r#monthly_occurrence: Box<Option<super::super::types::automation::SoftwareUpdateConfigurationScheduleMonthlyOccurrence>>,
    #[builder(into)]
    #[serde(rename = "nextRun")]
    pub r#next_run: Option<String>,
    #[builder(into)]
    #[serde(rename = "nextRunOffsetMinutes")]
    pub r#next_run_offset_minutes: Option<f64>,
    /// Start time of the schedule. Must be at least five minutes in the future. Defaults to seven minutes in the future from the time the resource is created.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    #[builder(into)]
    #[serde(rename = "startTimeOffsetMinutes")]
    pub r#start_time_offset_minutes: Option<f64>,
    /// The timezone of the start time. Defaults to `Etc/UTC`. For possible values see: <https://docs.microsoft.com/en-us/rest/api/maps/timezone/gettimezoneenumwindows>
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
}
