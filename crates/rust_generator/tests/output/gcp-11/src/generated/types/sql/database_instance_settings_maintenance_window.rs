#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseInstanceSettingsMaintenanceWindow {
    /// Day of week (`1-7`), starting on Monday
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: Option<i32>,
    /// Hour of day (`0-23`), ignored if `day` not set
    #[builder(into)]
    #[serde(rename = "hour")]
    pub r#hour: Option<i32>,
    /// Receive updates after one week (`canary`) or after two weeks (`stable`) or after five weeks (`week5`) of notification.
    #[builder(into)]
    #[serde(rename = "updateTrack")]
    pub r#update_track: Option<String>,
}
