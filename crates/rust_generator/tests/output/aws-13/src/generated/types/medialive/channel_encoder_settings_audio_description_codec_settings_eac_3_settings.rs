#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings {
    /// Sets the attenuation control.
    #[builder(into)]
    #[serde(rename = "attenuationControl")]
    pub r#attenuation_control: Option<String>,
    /// Average bitrate in bits/second.
    #[builder(into)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Option<f64>,
    /// Specifies the bitstream mode (bsmod) for the emitted AC-3 stream.
    #[builder(into)]
    #[serde(rename = "bitstreamMode")]
    pub r#bitstream_mode: Option<String>,
    /// Dolby Digital Plus coding mode.
    #[builder(into)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Option<String>,
    #[builder(into)]
    #[serde(rename = "dcFilter")]
    pub r#dc_filter: Option<String>,
    #[builder(into)]
    #[serde(rename = "dialnorm")]
    pub r#dialnorm: Option<i32>,
    /// Sets the Dolby dynamic range compression profile.
    #[builder(into)]
    #[serde(rename = "drcLine")]
    pub r#drc_line: Option<String>,
    /// Sets the profile for heavy Dolby dynamic range compression.
    #[builder(into)]
    #[serde(rename = "drcRf")]
    pub r#drc_rf: Option<String>,
    #[builder(into)]
    #[serde(rename = "lfeControl")]
    pub r#lfe_control: Option<String>,
    /// When set to enabled, applies a 120Hz lowpass filter to the LFE channel prior to encoding.
    #[builder(into)]
    #[serde(rename = "lfeFilter")]
    pub r#lfe_filter: Option<String>,
    #[builder(into)]
    #[serde(rename = "loRoCenterMixLevel")]
    pub r#lo_ro_center_mix_level: Option<f64>,
    #[builder(into)]
    #[serde(rename = "loRoSurroundMixLevel")]
    pub r#lo_ro_surround_mix_level: Option<f64>,
    #[builder(into)]
    #[serde(rename = "ltRtCenterMixLevel")]
    pub r#lt_rt_center_mix_level: Option<f64>,
    #[builder(into)]
    #[serde(rename = "ltRtSurroundMixLevel")]
    pub r#lt_rt_surround_mix_level: Option<f64>,
    /// Metadata control.
    #[builder(into)]
    #[serde(rename = "metadataControl")]
    pub r#metadata_control: Option<String>,
    #[builder(into)]
    #[serde(rename = "passthroughControl")]
    pub r#passthrough_control: Option<String>,
    #[builder(into)]
    #[serde(rename = "phaseControl")]
    pub r#phase_control: Option<String>,
    #[builder(into)]
    #[serde(rename = "stereoDownmix")]
    pub r#stereo_downmix: Option<String>,
    #[builder(into)]
    #[serde(rename = "surroundExMode")]
    pub r#surround_ex_mode: Option<String>,
    #[builder(into)]
    #[serde(rename = "surroundMode")]
    pub r#surround_mode: Option<String>,
}
