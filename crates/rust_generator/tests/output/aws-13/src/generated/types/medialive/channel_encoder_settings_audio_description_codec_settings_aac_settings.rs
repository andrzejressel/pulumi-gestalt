#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings {
    /// Average bitrate in bits/second.
    #[builder(into)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Option<f64>,
    /// Mono, Stereo, or 5.1 channel layout.
    #[builder(into)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Option<String>,
    /// Set to "broadcasterMixedAd" when input contains pre-mixed main audio + AD (narration) as a stereo pair.
    #[builder(into)]
    #[serde(rename = "inputType")]
    pub r#input_type: Option<String>,
    /// AAC profile.
    #[builder(into)]
    #[serde(rename = "profile")]
    pub r#profile: Option<String>,
    /// The rate control mode.
    #[builder(into)]
    #[serde(rename = "rateControlMode")]
    pub r#rate_control_mode: Option<String>,
    /// Sets LATM/LOAS AAC output for raw containers.
    #[builder(into)]
    #[serde(rename = "rawFormat")]
    pub r#raw_format: Option<String>,
    /// Sample rate in Hz.
    #[builder(into)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Option<f64>,
    /// Use MPEG-2 AAC audio instead of MPEG-4 AAC audio for raw or MPEG-2 Transport Stream containers.
    #[builder(into)]
    #[serde(rename = "spec")]
    pub r#spec: Option<String>,
    /// VBR Quality Level - Only used if rateControlMode is VBR.
    #[builder(into)]
    #[serde(rename = "vbrQuality")]
    pub r#vbr_quality: Option<String>,
}
