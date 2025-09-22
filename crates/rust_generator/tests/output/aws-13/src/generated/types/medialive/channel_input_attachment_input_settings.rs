#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelInputAttachmentInputSettings {
    /// Used to select the audio stream to decode for inputs that have multiple. See Audio Selectors for more details.
    #[builder(into)]
    #[serde(rename = "audioSelectors")]
    pub r#audio_selectors: Option<Vec<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelector>>,
    /// Used to select the caption input to use for inputs that have multiple available. See Caption Selectors for more details.
    #[builder(into)]
    #[serde(rename = "captionSelectors")]
    pub r#caption_selectors: Option<Vec<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelector>>,
    /// Enable or disable the deblock filter when filtering.
    #[builder(into)]
    #[serde(rename = "deblockFilter")]
    pub r#deblock_filter: Option<String>,
    /// Enable or disable the denoise filter when filtering.
    #[builder(into)]
    #[serde(rename = "denoiseFilter")]
    pub r#denoise_filter: Option<String>,
    /// Adjusts the magnitude of filtering from 1 (minimal) to 5 (strongest).
    #[builder(into)]
    #[serde(rename = "filterStrength")]
    pub r#filter_strength: Option<i32>,
    /// Turns on the filter for the input.
    #[builder(into)]
    #[serde(rename = "inputFilter")]
    pub r#input_filter: Option<String>,
    /// Input settings. See Network Input Settings for more details.
    #[builder(into)]
    #[serde(rename = "networkInputSettings")]
    pub r#network_input_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsNetworkInputSettings>>,
    /// PID from which to read SCTE-35 messages.
    #[builder(into)]
    #[serde(rename = "scte35Pid")]
    pub r#scte_35_pid: Option<i32>,
    /// Specifies whether to extract applicable ancillary data from a SMPTE-2038 source in the input.
    #[builder(into)]
    #[serde(rename = "smpte2038DataPreference")]
    pub r#smpte_2038_data_preference: Option<String>,
    /// Loop input if it is a file.
    #[builder(into)]
    #[serde(rename = "sourceEndBehavior")]
    pub r#source_end_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "videoSelector")]
    pub r#video_selector: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsVideoSelector>>,
}
