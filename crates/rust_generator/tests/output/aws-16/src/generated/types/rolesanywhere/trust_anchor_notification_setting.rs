#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TrustAnchorNotificationSetting {
    #[builder(into)]
    #[serde(rename = "channel")]
    pub r#channel: Option<String>,
    #[builder(into)]
    #[serde(rename = "configuredBy")]
    pub r#configured_by: Option<String>,
    /// Whether or not the Trust Anchor should be enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "event")]
    pub r#event: Option<String>,
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Option<i32>,
}
