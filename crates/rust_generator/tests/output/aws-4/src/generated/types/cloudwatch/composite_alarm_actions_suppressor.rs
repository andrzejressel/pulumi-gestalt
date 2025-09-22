#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CompositeAlarmActionsSuppressor {
    /// Can be an AlarmName or an Amazon Resource Name (ARN) from an existing alarm.
    #[builder(into)]
    #[serde(rename = "alarm")]
    pub r#alarm: String,
    /// The maximum time in seconds that the composite alarm waits after suppressor alarm goes out of the `ALARM` state. After this time, the composite alarm performs its actions.
    #[builder(into)]
    #[serde(rename = "extensionPeriod")]
    pub r#extension_period: i32,
    /// The maximum time in seconds that the composite alarm waits for the suppressor alarm to go into the `ALARM` state. After this time, the composite alarm performs its actions.
    #[builder(into)]
    #[serde(rename = "waitPeriod")]
    pub r#wait_period: i32,
}
