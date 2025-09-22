#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsWavSettings {
    #[builder(into)]
    #[serde(rename = "bitDepth")]
    pub r#bit_depth: Option<f64>,
    #[builder(into)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Option<String>,
    /// Sample rate in Hz.
    #[builder(into)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Option<f64>,
}
