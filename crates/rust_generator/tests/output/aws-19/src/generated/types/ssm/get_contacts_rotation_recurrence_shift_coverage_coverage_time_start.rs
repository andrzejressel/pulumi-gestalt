#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetContactsRotationRecurrenceShiftCoverageCoverageTimeStart {
    #[builder(into)]
    #[serde(rename = "hourOfDay")]
    pub r#hour_of_day: i32,
    #[builder(into)]
    #[serde(rename = "minuteOfHour")]
    pub r#minute_of_hour: i32,
}
