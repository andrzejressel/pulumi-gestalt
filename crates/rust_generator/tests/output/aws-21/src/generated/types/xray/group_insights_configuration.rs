#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupInsightsConfiguration {
    /// Specifies whether insights are enabled.
    #[builder(into)]
    #[serde(rename = "insightsEnabled")]
    pub r#insights_enabled: bool,
    /// Specifies whether insight notifications are enabled.
    #[builder(into)]
    #[serde(rename = "notificationsEnabled")]
    pub r#notifications_enabled: Option<bool>,
}
