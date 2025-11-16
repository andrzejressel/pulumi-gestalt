#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationAutoStopConfiguration {
    /// Enables the application to automatically stop after a certain amount of time being idle. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The amount of idle time in minutes after which your application will automatically stop. Defaults to `15` minutes.
    #[builder(into)]
    #[serde(rename = "idleTimeoutMinutes")]
    pub r#idle_timeout_minutes: Option<i32>,
}
