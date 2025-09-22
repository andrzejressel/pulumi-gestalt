#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetContactsRotationRecurrence {
    #[builder(into)]
    #[serde(rename = "dailySettings")]
    pub r#daily_settings: Vec<super::super::types::ssm::GetContactsRotationRecurrenceDailySetting>,
    #[builder(into)]
    #[serde(rename = "monthlySettings")]
    pub r#monthly_settings: Vec<super::super::types::ssm::GetContactsRotationRecurrenceMonthlySetting>,
    #[builder(into)]
    #[serde(rename = "numberOfOnCalls")]
    pub r#number_of_on_calls: i32,
    #[builder(into)]
    #[serde(rename = "recurrenceMultiplier")]
    pub r#recurrence_multiplier: i32,
    #[builder(into)]
    #[serde(rename = "shiftCoverages")]
    pub r#shift_coverages: Vec<super::super::types::ssm::GetContactsRotationRecurrenceShiftCoverage>,
    #[builder(into)]
    #[serde(rename = "weeklySettings")]
    pub r#weekly_settings: Vec<super::super::types::ssm::GetContactsRotationRecurrenceWeeklySetting>,
}
