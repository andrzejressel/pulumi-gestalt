#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings {
    /// Average bitrate in bits/second.
    #[builder(into)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Option<f64>,
    /// Dolby Digital Plus with Dolby Atmos coding mode.
    #[builder(into)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Option<String>,
    /// Sets the dialnorm for the output.
    #[builder(into)]
    #[serde(rename = "dialnorm")]
    pub r#dialnorm: Option<f64>,
    /// Sets the Dolby dynamic range compression profile.
    #[builder(into)]
    #[serde(rename = "drcLine")]
    pub r#drc_line: Option<String>,
    /// Sets the profile for heavy Dolby dynamic range compression.
    #[builder(into)]
    #[serde(rename = "drcRf")]
    pub r#drc_rf: Option<String>,
    /// Height dimensional trim.
    #[builder(into)]
    #[serde(rename = "heightTrim")]
    pub r#height_trim: Option<f64>,
    /// Surround dimensional trim.
    #[builder(into)]
    #[serde(rename = "surroundTrim")]
    pub r#surround_trim: Option<f64>,
}
