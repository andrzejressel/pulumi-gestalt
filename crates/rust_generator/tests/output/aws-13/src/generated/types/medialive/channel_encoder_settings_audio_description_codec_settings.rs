#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettings {
    /// Aac Settings. See AAC Settings for more details.
    #[builder(into)]
    #[serde(rename = "aacSettings")]
    pub r#aac_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings>>,
    /// Ac3 Settings. See AC3 Settings for more details.
    #[builder(into)]
    #[serde(rename = "ac3Settings")]
    pub r#ac_3_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings>>,
    /// Eac3 Atmos Settings. See EAC3 Atmos Settings
    #[builder(into)]
    #[serde(rename = "eac3AtmosSettings")]
    pub r#eac_3_atmos_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings>>,
    /// Eac3 Settings. See EAC3 Settings
    #[builder(into)]
    #[serde(rename = "eac3Settings")]
    pub r#eac_3_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings>>,
    #[builder(into)]
    #[serde(rename = "mp2Settings")]
    pub r#mp_2_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsMp2Settings>>,
    #[builder(into)]
    #[serde(rename = "passThroughSettings")]
    pub r#pass_through_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsPassThroughSettings>>,
    #[builder(into)]
    #[serde(rename = "wavSettings")]
    pub r#wav_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsWavSettings>>,
}
