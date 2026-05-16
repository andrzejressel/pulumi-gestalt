#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTriggerScheduleScheduleMonthly {
    /// The occurrence of the specified day during the month.
    #[builder(into)]
    #[serde(rename = "week")]
    pub r#week: i32,
    /// The day of the week on which the trigger runs.
    #[builder(into)]
    #[serde(rename = "weekday")]
    pub r#weekday: String,
}
