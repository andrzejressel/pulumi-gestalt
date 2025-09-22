#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlexibleServerMaintenanceWindow {
    /// The day of week for maintenance window. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Option<i32>,
    /// The start hour for maintenance window. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "startHour")]
    pub r#start_hour: Option<i32>,
    /// The start minute for maintenance window. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "startMinute")]
    pub r#start_minute: Option<i32>,
}
