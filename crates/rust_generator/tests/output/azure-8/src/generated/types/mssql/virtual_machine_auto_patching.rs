#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualMachineAutoPatching {
    /// The day of week to apply the patch on. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: String,
    /// The size of the Maintenance Window in minutes.
    #[builder(into)]
    #[serde(rename = "maintenanceWindowDurationInMinutes")]
    pub r#maintenance_window_duration_in_minutes: i32,
    /// The Hour, in the Virtual Machine Time-Zone when the patching maintenance window should begin.
    #[builder(into)]
    #[serde(rename = "maintenanceWindowStartingHour")]
    pub r#maintenance_window_starting_hour: i32,
}
