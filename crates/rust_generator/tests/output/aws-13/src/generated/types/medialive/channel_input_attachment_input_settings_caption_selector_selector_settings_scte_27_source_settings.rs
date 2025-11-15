#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte27SourceSettings {
    #[builder(into)]
    #[serde(rename = "ocrLanguage")]
    pub r#ocr_language: Option<String>,
    #[builder(into)]
    #[serde(rename = "pid")]
    pub r#pid: Option<i32>,
}
