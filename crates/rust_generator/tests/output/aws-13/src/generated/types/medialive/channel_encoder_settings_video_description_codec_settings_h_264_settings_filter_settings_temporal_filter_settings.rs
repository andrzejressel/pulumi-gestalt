#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettingsTemporalFilterSettings {
    /// Post filter sharpening.
    #[builder(into)]
    #[serde(rename = "postFilterSharpening")]
    pub r#post_filter_sharpening: Option<String>,
    /// Filter strength.
    #[builder(into)]
    #[serde(rename = "strength")]
    pub r#strength: Option<String>,
}
