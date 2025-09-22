#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsAudioDescriptionAudioNormalizationSettings {
    /// Audio normalization algorithm to use. itu17701 conforms to the CALM Act specification, itu17702 to the EBU R-128 specification.
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Option<String>,
    /// Algorithm control for the audio description.
    #[builder(into)]
    #[serde(rename = "algorithmControl")]
    pub r#algorithm_control: Option<String>,
    /// Target LKFS (loudness) to adjust volume to.
    #[builder(into)]
    #[serde(rename = "targetLkfs")]
    pub r#target_lkfs: Option<f64>,
}
