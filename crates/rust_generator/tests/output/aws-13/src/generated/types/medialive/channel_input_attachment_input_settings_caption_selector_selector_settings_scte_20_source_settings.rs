#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte20SourceSettings {
    #[builder(into)]
    #[serde(rename = "convert608To708")]
    pub r#convert_608_to_708: Option<String>,
    #[builder(into)]
    #[serde(rename = "source608ChannelNumber")]
    pub r#source_608_channel_number: Option<i32>,
}
