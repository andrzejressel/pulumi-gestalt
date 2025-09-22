#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettings {
    /// Audio HLS Rendition Selection. See Audio HLS Rendition Selection for more details.
    #[builder(into)]
    #[serde(rename = "audioHlsRenditionSelection")]
    pub r#audio_hls_rendition_selection: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioHlsRenditionSelection>>,
    /// Audio Language Selection. See Audio Language Selection for more details.
    #[builder(into)]
    #[serde(rename = "audioLanguageSelection")]
    pub r#audio_language_selection: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioLanguageSelection>>,
    /// Audio Pid Selection. See Audio PID Selection for more details.
    #[builder(into)]
    #[serde(rename = "audioPidSelection")]
    pub r#audio_pid_selection: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioPidSelection>>,
    /// Audio Track Selection. See Audio Track Selection for more details.
    #[builder(into)]
    #[serde(rename = "audioTrackSelection")]
    pub r#audio_track_selection: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelection>>,
}
