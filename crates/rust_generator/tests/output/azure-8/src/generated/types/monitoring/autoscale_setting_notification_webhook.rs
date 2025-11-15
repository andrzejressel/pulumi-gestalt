#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutoscaleSettingNotificationWebhook {
    /// A map of settings.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
    /// The HTTPS URI which should receive scale notifications.
    #[builder(into)]
    #[serde(rename = "serviceUri")]
    pub r#service_uri: String,
}
