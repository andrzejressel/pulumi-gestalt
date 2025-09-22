#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScheduleFlexibleTimeWindow {
    /// Maximum time window during which a schedule can be invoked. Ranges from `1` to `1440` minutes.
    #[builder(into)]
    #[serde(rename = "maximumWindowInMinutes")]
    pub r#maximum_window_in_minutes: Option<i32>,
    /// Determines whether the schedule is invoked within a flexible time window. One of: `OFF`, `FLEXIBLE`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
}
