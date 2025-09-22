#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelMaintenance {
    /// The day of the week to use for maintenance.
    #[builder(into)]
    #[serde(rename = "maintenanceDay")]
    pub r#maintenance_day: String,
    /// The hour maintenance will start.
    #[builder(into)]
    #[serde(rename = "maintenanceStartTime")]
    pub r#maintenance_start_time: String,
}
