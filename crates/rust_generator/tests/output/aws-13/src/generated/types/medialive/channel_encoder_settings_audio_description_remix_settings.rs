#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsAudioDescriptionRemixSettings {
    #[builder(into)]
    #[serde(rename = "channelMappings")]
    pub r#channel_mappings: Vec<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMapping>,
    #[builder(into)]
    #[serde(rename = "channelsIn")]
    pub r#channels_in: Option<i32>,
    #[builder(into)]
    #[serde(rename = "channelsOut")]
    pub r#channels_out: Option<i32>,
}
