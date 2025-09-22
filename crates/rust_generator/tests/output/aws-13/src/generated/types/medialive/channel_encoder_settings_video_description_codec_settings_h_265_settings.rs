#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings {
    /// Enables or disables adaptive quantization.
    #[builder(into)]
    #[serde(rename = "adaptiveQuantization")]
    pub r#adaptive_quantization: Option<String>,
    /// Indicates that AFD values will be written into the output stream.
    #[builder(into)]
    #[serde(rename = "afdSignaling")]
    pub r#afd_signaling: Option<String>,
    /// Whether or not EML should insert an Alternative Transfer Function SEI message.
    #[builder(into)]
    #[serde(rename = "alternativeTransferFunction")]
    pub r#alternative_transfer_function: Option<String>,
    /// Average bitrate in bits/second.
    #[builder(into)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: i32,
    /// Size of buffer in bits.
    #[builder(into)]
    #[serde(rename = "bufSize")]
    pub r#buf_size: Option<i32>,
    /// Includes color space metadata in the output.
    #[builder(into)]
    #[serde(rename = "colorMetadata")]
    pub r#color_metadata: Option<String>,
    /// Define the color metadata for the output. H265 Color Space Settings for more details.
    #[builder(into)]
    #[serde(rename = "colorSpaceSettings")]
    pub r#color_space_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettings>>,
    /// Filters to apply to an encode. See H265 Filter Settings for more details.
    #[builder(into)]
    #[serde(rename = "filterSettings")]
    pub r#filter_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettings>>,
    /// Four bit AFD value to write on all frames of video in the output stream.
    #[builder(into)]
    #[serde(rename = "fixedAfd")]
    pub r#fixed_afd: Option<String>,
    #[builder(into)]
    #[serde(rename = "flickerAq")]
    pub r#flicker_aq: Option<String>,
    /// Framerate denominator.
    #[builder(into)]
    #[serde(rename = "framerateDenominator")]
    pub r#framerate_denominator: i32,
    /// Framerate numerator.
    #[builder(into)]
    #[serde(rename = "framerateNumerator")]
    pub r#framerate_numerator: i32,
    /// Frequency of closed GOPs.
    #[builder(into)]
    #[serde(rename = "gopClosedCadence")]
    pub r#gop_closed_cadence: Option<i32>,
    /// GOP size in units of either frames of seconds per `gop_size_units`.
    #[builder(into)]
    #[serde(rename = "gopSize")]
    pub r#gop_size: Option<f64>,
    /// Indicates if the `gop_size` is specified in frames or seconds.
    #[builder(into)]
    #[serde(rename = "gopSizeUnits")]
    pub r#gop_size_units: Option<String>,
    /// H265 level.
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
    /// Set the minimum QP.
    #[builder(into)]
    #[serde(rename = "minQp")]
    pub r#min_qp: Option<i32>,
    /// Enables or disables motion vector over picture boundaries.
    #[builder(into)]
    #[serde(rename = "mvOverPictureBoundaries")]
    pub r#mv_over_picture_boundaries: Option<String>,
    /// Enables or disables the motion vector temporal predictor.
    #[builder(into)]
    #[serde(rename = "mvTemporalPredictor")]
    pub r#mv_temporal_predictor: Option<String>,
    /// Pixel Aspect Ratio denominator.
    #[builder(into)]
    #[serde(rename = "parDenominator")]
    pub r#par_denominator: Option<i32>,
    /// Pixel Aspect Ratio numerator.
    #[builder(into)]
    #[serde(rename = "parNumerator")]
    pub r#par_numerator: Option<i32>,
    /// H265 profile.
    #[builder(into)]
    #[serde(rename = "profile")]
    pub r#profile: Option<String>,
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
    /// Set the H265 tier in the output.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Option<String>,
    /// Sets the height of tiles.
    #[builder(into)]
    #[serde(rename = "tileHeight")]
    pub r#tile_height: Option<i32>,
    /// Enables or disables padding of tiles.
    #[builder(into)]
    #[serde(rename = "tilePadding")]
    pub r#tile_padding: Option<String>,
    /// Sets the width of tiles.
    #[builder(into)]
    #[serde(rename = "tileWidth")]
    pub r#tile_width: Option<i32>,
    /// Apply a burned in timecode. See H265 Timecode Burnin Settings for more details.
    #[builder(into)]
    #[serde(rename = "timecodeBurninSettings")]
    pub r#timecode_burnin_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsTimecodeBurninSettings>>,
    /// Determines how timecodes should be inserted into the video elementary stream.
    #[builder(into)]
    #[serde(rename = "timecodeInsertion")]
    pub r#timecode_insertion: Option<String>,
    /// Sets the size of the treeblock.
    #[builder(into)]
    #[serde(rename = "treeblockSize")]
    pub r#treeblock_size: Option<String>,
}
