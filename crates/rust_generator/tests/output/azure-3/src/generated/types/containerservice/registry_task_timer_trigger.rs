#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistryTaskTimerTrigger {
    /// Should the trigger be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The name which should be used for this trigger.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The CRON expression for the task schedule.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: String,
}
