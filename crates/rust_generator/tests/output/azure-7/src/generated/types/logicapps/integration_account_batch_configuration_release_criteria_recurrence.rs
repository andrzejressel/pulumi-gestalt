#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntegrationAccountBatchConfigurationReleaseCriteriaRecurrence {
    /// The end time of the schedule, formatted as an RFC3339 string.
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Option<String>,
    /// The frequency of the schedule. Possible values are `Day`, `Hour`, `Minute`, `Month`, `NotSpecified`, `Second`, `Week` and `Year`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: String,
    /// The number of `frequency`s between runs.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: i32,
    /// A `schedule` block as documented below.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Option<Box<super::super::types::logicapps::IntegrationAccountBatchConfigurationReleaseCriteriaRecurrenceSchedule>>,
    /// The start time of the schedule, formatted as an RFC3339 string.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    /// The timezone of the start/end time.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
}
