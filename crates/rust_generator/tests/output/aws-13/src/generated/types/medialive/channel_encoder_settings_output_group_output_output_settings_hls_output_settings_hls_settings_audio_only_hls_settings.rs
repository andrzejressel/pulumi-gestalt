#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettings {
    #[builder(into)]
    #[serde(rename = "audioGroupId")]
    pub r#audio_group_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "audioOnlyImage")]
    pub r#audio_only_image: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettingsAudioOnlyImage>>,
    #[builder(into)]
    #[serde(rename = "audioTrackType")]
    pub r#audio_track_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "segmentType")]
    pub r#segment_type: Option<String>,
}
