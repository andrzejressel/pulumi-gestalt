#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertProcessingRuleActionGroupScheduleRecurrence {
    /// One or more `daily` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "dailies")]
    pub r#dailies: Box<Option<Vec<super::super::types::monitoring::AlertProcessingRuleActionGroupScheduleRecurrenceDaily>>>,
    /// One or more `monthly` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "monthlies")]
    pub r#monthlies: Box<Option<Vec<super::super::types::monitoring::AlertProcessingRuleActionGroupScheduleRecurrenceMonthly>>>,
    /// One or more `weekly` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "weeklies")]
    pub r#weeklies: Box<Option<Vec<super::super::types::monitoring::AlertProcessingRuleActionGroupScheduleRecurrenceWeekly>>>,
}
