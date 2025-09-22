#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScheduleNotificationSettings {
    /// The status of the notification. Possible values are `Enabled` and `Disabled`. Defaults to `Disabled`
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// Time in minutes before event at which notification will be sent.
    #[builder(into)]
    #[serde(rename = "timeInMinutes")]
    pub r#time_in_minutes: Option<i32>,
    /// The webhook URL to which the notification will be sent.
    #[builder(into)]
    #[serde(rename = "webhookUrl")]
    pub r#webhook_url: Option<String>,
}
