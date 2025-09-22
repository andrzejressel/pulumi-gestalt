#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsTimecodeBurninSettings {
    /// Set a prefix on the burned in timecode.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Sets the size of the burned in timecode.
    #[builder(into)]
    #[serde(rename = "timecodeBurninFontSize")]
    pub r#timecode_burnin_font_size: Option<String>,
    /// Sets the position of the burned in timecode.
    #[builder(into)]
    #[serde(rename = "timecodeBurninPosition")]
    pub r#timecode_burnin_position: Option<String>,
}
