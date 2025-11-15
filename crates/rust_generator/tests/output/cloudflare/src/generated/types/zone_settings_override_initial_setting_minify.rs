#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZoneSettingsOverrideInitialSettingMinify {
    #[builder(into)]
    #[serde(rename = "css")]
    pub r#css: String,
    #[builder(into)]
    #[serde(rename = "html")]
    pub r#html: String,
    #[builder(into)]
    #[serde(rename = "js")]
    pub r#js: String,
}
