#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateConfigElementaryStreamVideoStreamH264 {
    /// The video bitrate in bits per second.
    #[builder(into)]
    #[serde(rename = "bitrateBps")]
    pub r#bitrate_bps: i32,
    /// Target CRF level. The default is '21'.
    #[builder(into)]
    #[serde(rename = "crfLevel")]
    pub r#crf_level: Option<i32>,
    /// The entropy coder to use. The default is 'cabac'.
    #[builder(into)]
    #[serde(rename = "entropyCoder")]
    pub r#entropy_coder: Option<String>,
    /// The target video frame rate in frames per second (FPS).
    #[builder(into)]
    #[serde(rename = "frameRate")]
    pub r#frame_rate: i32,
    /// Select the GOP size based on the specified duration. The default is '3s'.
    #[builder(into)]
    #[serde(rename = "gopDuration")]
    pub r#gop_duration: Option<String>,
    /// The height of the video in pixels.
    #[builder(into)]
    #[serde(rename = "heightPixels")]
    pub r#height_pixels: Option<i32>,
    /// HLG color format setting for H264.
    #[builder(into)]
    #[serde(rename = "hlg")]
    pub r#hlg: Option<Box<super::super::types::transcoder::JobTemplateConfigElementaryStreamVideoStreamH264Hlg>>,
    /// Pixel format to use. The default is 'yuv420p'.
    #[builder(into)]
    #[serde(rename = "pixelFormat")]
    pub r#pixel_format: Option<String>,
    /// Enforces the specified codec preset. The default is 'veryfast'.
    #[builder(into)]
    #[serde(rename = "preset")]
    pub r#preset: Option<String>,
    /// Enforces the specified codec profile.
    #[builder(into)]
    #[serde(rename = "profile")]
    pub r#profile: Option<String>,
    /// Specify the mode. The default is 'vbr'.
    #[builder(into)]
    #[serde(rename = "rateControlMode")]
    pub r#rate_control_mode: Option<String>,
    /// SDR color format setting for H264.
    #[builder(into)]
    #[serde(rename = "sdr")]
    pub r#sdr: Option<Box<super::super::types::transcoder::JobTemplateConfigElementaryStreamVideoStreamH264Sdr>>,
    /// Initial fullness of the Video Buffering Verifier (VBV) buffer in bits.
    #[builder(into)]
    #[serde(rename = "vbvFullnessBits")]
    pub r#vbv_fullness_bits: Option<i32>,
    /// Size of the Video Buffering Verifier (VBV) buffer in bits.
    #[builder(into)]
    #[serde(rename = "vbvSizeBits")]
    pub r#vbv_size_bits: Option<i32>,
    /// The width of the video in pixels.
    #[builder(into)]
    #[serde(rename = "widthPixels")]
    pub r#width_pixels: Option<i32>,
}
