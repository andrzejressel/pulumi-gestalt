#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsCaptionLanguageMapping {
    #[builder(into)]
    #[serde(rename = "captionChannel")]
    pub r#caption_channel: i32,
    #[builder(into)]
    #[serde(rename = "languageCode")]
    pub r#language_code: String,
    /// Human readable information to indicate captions available for players (eg. English, or Spanish).
    #[builder(into)]
    #[serde(rename = "languageDescription")]
    pub r#language_description: String,
}
