#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsVideoDescription {
    /// The video codec settings. See Video Codec Settings for more details.
    #[builder(into)]
    #[serde(rename = "codecSettings")]
    pub r#codec_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettings>>,
    /// Output video height in pixels.
    #[builder(into)]
    #[serde(rename = "height")]
    pub r#height: Option<i32>,
    /// The name of the video description.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Indicate how to respond to the AFD values that might be in the input video.
    #[builder(into)]
    #[serde(rename = "respondToAfd")]
    pub r#respond_to_afd: Option<String>,
    /// Behavior on how to scale.
    #[builder(into)]
    #[serde(rename = "scalingBehavior")]
    pub r#scaling_behavior: Option<String>,
    /// Changes the strength of the anti-alias filter used for scaling.
    #[builder(into)]
    #[serde(rename = "sharpness")]
    pub r#sharpness: Option<i32>,
    /// Output video width in pixels.
    #[builder(into)]
    #[serde(rename = "width")]
    pub r#width: Option<i32>,
}
