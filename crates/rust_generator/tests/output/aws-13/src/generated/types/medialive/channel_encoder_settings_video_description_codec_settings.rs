#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettings {
    /// Frame capture settings. See Frame Capture Settings for more details.
    #[builder(into)]
    #[serde(rename = "frameCaptureSettings")]
    pub r#frame_capture_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsFrameCaptureSettings>>,
    /// H264 settings. See H264 Settings for more details.
    #[builder(into)]
    #[serde(rename = "h264Settings")]
    pub r#h_264_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings>>,
    #[builder(into)]
    #[serde(rename = "h265Settings")]
    pub r#h_265_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings>>,
}
