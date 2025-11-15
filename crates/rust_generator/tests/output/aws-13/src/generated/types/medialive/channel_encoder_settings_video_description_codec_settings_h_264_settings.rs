#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings {
    /// Enables or disables adaptive quantization.
    #[builder(into)]
    #[serde(rename = "adaptiveQuantization")]
    pub r#adaptive_quantization: Option<String>,
    /// Indicates that AFD values will be written into the output stream.
    #[builder(into)]
    #[serde(rename = "afdSignaling")]
    pub r#afd_signaling: Option<String>,
    /// Average bitrate in bits/second.
    #[builder(into)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Option<i32>,
    #[builder(into)]
    #[serde(rename = "bufFillPct")]
    pub r#buf_fill_pct: Option<i32>,
    /// Size of buffer in bits.
    #[builder(into)]
    #[serde(rename = "bufSize")]
    pub r#buf_size: Option<i32>,
    /// Includes color space metadata in the output.
    #[builder(into)]
    #[serde(rename = "colorMetadata")]
    pub r#color_metadata: Option<String>,
    /// Entropy encoding mode.
    #[builder(into)]
    #[serde(rename = "entropyEncoding")]
    pub r#entropy_encoding: Option<String>,
    /// Filters to apply to an encode. See H264 Filter Settings for more details.
    #[builder(into)]
    #[serde(rename = "filterSettings")]
    pub r#filter_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettings>>,
    /// Four bit AFD value to write on all frames of video in the output stream.
    #[builder(into)]
    #[serde(rename = "fixedAfd")]
    pub r#fixed_afd: Option<String>,
    #[builder(into)]
    #[serde(rename = "flickerAq")]
    pub r#flicker_aq: Option<String>,
    /// Controls whether coding is performed on a field basis or on a frame basis.
    #[builder(into)]
    #[serde(rename = "forceFieldPictures")]
    pub r#force_field_pictures: Option<String>,
    /// Indicates how the output video frame rate is specified.
    #[builder(into)]
    #[serde(rename = "framerateControl")]
    pub r#framerate_control: Option<String>,
    /// Framerate denominator.
    #[builder(into)]
    #[serde(rename = "framerateDenominator")]
    pub r#framerate_denominator: Option<i32>,
    /// Framerate numerator.
    #[builder(into)]
    #[serde(rename = "framerateNumerator")]
    pub r#framerate_numerator: Option<i32>,
    /// GOP-B reference.
    #[builder(into)]
    #[serde(rename = "gopBReference")]
    pub r#gop_b_reference: Option<String>,
    /// Frequency of closed GOPs.
    #[builder(into)]
    #[serde(rename = "gopClosedCadence")]
    pub r#gop_closed_cadence: Option<i32>,
    /// Number of B-frames between reference frames.
    #[builder(into)]
    #[serde(rename = "gopNumBFrames")]
    pub r#gop_num_b_frames: Option<i32>,
    /// GOP size in units of either frames of seconds per `gop_size_units`.
    #[builder(into)]
    #[serde(rename = "gopSize")]
    pub r#gop_size: Option<f64>,
    /// Indicates if the `gop_size` is specified in frames or seconds.
    #[builder(into)]
    #[serde(rename = "gopSizeUnits")]
    pub r#gop_size_units: Option<String>,
    /// H264 level.
    #[builder(into)]
    #[serde(rename = "level")]
    pub r#level: Option<String>,
    /// Amount of lookahead.
    #[builder(into)]
    #[serde(rename = "lookAheadRateControl")]
    pub r#look_ahead_rate_control: Option<String>,
    /// Set the maximum bitrate in order to accommodate expected spikes in the complexity of the video.
    #[builder(into)]
    #[serde(rename = "maxBitrate")]
    pub r#max_bitrate: Option<i32>,
    /// Min interval.
    #[builder(into)]
    #[serde(rename = "minIInterval")]
    pub r#min_i_interval: Option<i32>,
    /// Number of reference frames to use.
    #[builder(into)]
    #[serde(rename = "numRefFrames")]
    pub r#num_ref_frames: Option<i32>,
    /// Indicates how the output pixel aspect ratio is specified.
    #[builder(into)]
    #[serde(rename = "parControl")]
    pub r#par_control: Option<String>,
    /// Pixel Aspect Ratio denominator.
    #[builder(into)]
    #[serde(rename = "parDenominator")]
    pub r#par_denominator: Option<i32>,
    /// Pixel Aspect Ratio numerator.
    #[builder(into)]
    #[serde(rename = "parNumerator")]
    pub r#par_numerator: Option<i32>,
    /// H264 profile.
    #[builder(into)]
    #[serde(rename = "profile")]
    pub r#profile: Option<String>,
    /// Quality level.
    #[builder(into)]
    #[serde(rename = "qualityLevel")]
    pub r#quality_level: Option<String>,
    /// Controls the target quality for the video encode.
    #[builder(into)]
    #[serde(rename = "qvbrQualityLevel")]
    pub r#qvbr_quality_level: Option<i32>,
    /// Rate control mode.
    #[builder(into)]
    #[serde(rename = "rateControlMode")]
    pub r#rate_control_mode: Option<String>,
    /// Sets the scan type of the output.
    #[builder(into)]
    #[serde(rename = "scanType")]
    pub r#scan_type: Option<String>,
    /// Scene change detection.
    #[builder(into)]
    #[serde(rename = "sceneChangeDetect")]
    pub r#scene_change_detect: Option<String>,
    /// Number of slices per picture.
    #[builder(into)]
    #[serde(rename = "slices")]
    pub r#slices: Option<i32>,
    /// Softness.
    #[builder(into)]
    #[serde(rename = "softness")]
    pub r#softness: Option<i32>,
    /// Makes adjustments within each frame based on spatial variation of content complexity.
    #[builder(into)]
    #[serde(rename = "spatialAq")]
    pub r#spatial_aq: Option<String>,
    /// Subgop length.
    #[builder(into)]
    #[serde(rename = "subgopLength")]
    pub r#subgop_length: Option<String>,
    /// Produces a bitstream compliant with SMPTE RP-2027.
    #[builder(into)]
    #[serde(rename = "syntax")]
    pub r#syntax: Option<String>,
    /// Makes adjustments within each frame based on temporal variation of content complexity.
    #[builder(into)]
    #[serde(rename = "temporalAq")]
    pub r#temporal_aq: Option<String>,
    /// Determines how timecodes should be inserted into the video elementary stream.
    #[builder(into)]
    #[serde(rename = "timecodeInsertion")]
    pub r#timecode_insertion: Option<String>,
}
