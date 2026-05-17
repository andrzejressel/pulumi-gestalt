#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "adaptive_quantization",
                    &self.r#adaptive_quantization,
                ),
                to_pulumi_object_field(
                    "afd_signaling",
                    &self.r#afd_signaling,
                ),
                to_pulumi_object_field(
                    "alternative_transfer_function",
                    &self.r#alternative_transfer_function,
                ),
                to_pulumi_object_field(
                    "bitrate",
                    &self.r#bitrate,
                ),
                to_pulumi_object_field(
                    "buf_size",
                    &self.r#buf_size,
                ),
                to_pulumi_object_field(
                    "color_metadata",
                    &self.r#color_metadata,
                ),
                to_pulumi_object_field(
                    "color_space_settings",
                    &self.r#color_space_settings,
                ),
                to_pulumi_object_field(
                    "filter_settings",
                    &self.r#filter_settings,
                ),
                to_pulumi_object_field(
                    "fixed_afd",
                    &self.r#fixed_afd,
                ),
                to_pulumi_object_field(
                    "flicker_aq",
                    &self.r#flicker_aq,
                ),
                to_pulumi_object_field(
                    "framerate_denominator",
                    &self.r#framerate_denominator,
                ),
                to_pulumi_object_field(
                    "framerate_numerator",
                    &self.r#framerate_numerator,
                ),
                to_pulumi_object_field(
                    "gop_closed_cadence",
                    &self.r#gop_closed_cadence,
                ),
                to_pulumi_object_field(
                    "gop_size",
                    &self.r#gop_size,
                ),
                to_pulumi_object_field(
                    "gop_size_units",
                    &self.r#gop_size_units,
                ),
                to_pulumi_object_field(
                    "level",
                    &self.r#level,
                ),
                to_pulumi_object_field(
                    "look_ahead_rate_control",
                    &self.r#look_ahead_rate_control,
                ),
                to_pulumi_object_field(
                    "max_bitrate",
                    &self.r#max_bitrate,
                ),
                to_pulumi_object_field(
                    "min_i_interval",
                    &self.r#min_i_interval,
                ),
                to_pulumi_object_field(
                    "min_qp",
                    &self.r#min_qp,
                ),
                to_pulumi_object_field(
                    "mv_over_picture_boundaries",
                    &self.r#mv_over_picture_boundaries,
                ),
                to_pulumi_object_field(
                    "mv_temporal_predictor",
                    &self.r#mv_temporal_predictor,
                ),
                to_pulumi_object_field(
                    "par_denominator",
                    &self.r#par_denominator,
                ),
                to_pulumi_object_field(
                    "par_numerator",
                    &self.r#par_numerator,
                ),
                to_pulumi_object_field(
                    "profile",
                    &self.r#profile,
                ),
                to_pulumi_object_field(
                    "qvbr_quality_level",
                    &self.r#qvbr_quality_level,
                ),
                to_pulumi_object_field(
                    "rate_control_mode",
                    &self.r#rate_control_mode,
                ),
                to_pulumi_object_field(
                    "scan_type",
                    &self.r#scan_type,
                ),
                to_pulumi_object_field(
                    "scene_change_detect",
                    &self.r#scene_change_detect,
                ),
                to_pulumi_object_field(
                    "slices",
                    &self.r#slices,
                ),
                to_pulumi_object_field(
                    "tier",
                    &self.r#tier,
                ),
                to_pulumi_object_field(
                    "tile_height",
                    &self.r#tile_height,
                ),
                to_pulumi_object_field(
                    "tile_padding",
                    &self.r#tile_padding,
                ),
                to_pulumi_object_field(
                    "tile_width",
                    &self.r#tile_width,
                ),
                to_pulumi_object_field(
                    "timecode_burnin_settings",
                    &self.r#timecode_burnin_settings,
                ),
                to_pulumi_object_field(
                    "timecode_insertion",
                    &self.r#timecode_insertion,
                ),
                to_pulumi_object_field(
                    "treeblock_size",
                    &self.r#treeblock_size,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#adaptive_quantization: {
                        let field_value = match fields_map.get("adaptive_quantization") {
                            Some(value) => value,
                            None => bail!("Missing field 'adaptive_quantization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#afd_signaling: {
                        let field_value = match fields_map.get("afd_signaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'afd_signaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#alternative_transfer_function: {
                        let field_value = match fields_map.get("alternative_transfer_function") {
                            Some(value) => value,
                            None => bail!("Missing field 'alternative_transfer_function' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bitrate: {
                        let field_value = match fields_map.get("bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#buf_size: {
                        let field_value = match fields_map.get("buf_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'buf_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#color_metadata: {
                        let field_value = match fields_map.get("color_metadata") {
                            Some(value) => value,
                            None => bail!("Missing field 'color_metadata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#color_space_settings: {
                        let field_value = match fields_map.get("color_space_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'color_space_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_settings: {
                        let field_value = match fields_map.get("filter_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fixed_afd: {
                        let field_value = match fields_map.get("fixed_afd") {
                            Some(value) => value,
                            None => bail!("Missing field 'fixed_afd' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flicker_aq: {
                        let field_value = match fields_map.get("flicker_aq") {
                            Some(value) => value,
                            None => bail!("Missing field 'flicker_aq' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#framerate_denominator: {
                        let field_value = match fields_map.get("framerate_denominator") {
                            Some(value) => value,
                            None => bail!("Missing field 'framerate_denominator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#framerate_numerator: {
                        let field_value = match fields_map.get("framerate_numerator") {
                            Some(value) => value,
                            None => bail!("Missing field 'framerate_numerator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gop_closed_cadence: {
                        let field_value = match fields_map.get("gop_closed_cadence") {
                            Some(value) => value,
                            None => bail!("Missing field 'gop_closed_cadence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gop_size: {
                        let field_value = match fields_map.get("gop_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'gop_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gop_size_units: {
                        let field_value = match fields_map.get("gop_size_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'gop_size_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level: {
                        let field_value = match fields_map.get("level") {
                            Some(value) => value,
                            None => bail!("Missing field 'level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#look_ahead_rate_control: {
                        let field_value = match fields_map.get("look_ahead_rate_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'look_ahead_rate_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_bitrate: {
                        let field_value = match fields_map.get("max_bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_i_interval: {
                        let field_value = match fields_map.get("min_i_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_i_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_qp: {
                        let field_value = match fields_map.get("min_qp") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_qp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mv_over_picture_boundaries: {
                        let field_value = match fields_map.get("mv_over_picture_boundaries") {
                            Some(value) => value,
                            None => bail!("Missing field 'mv_over_picture_boundaries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mv_temporal_predictor: {
                        let field_value = match fields_map.get("mv_temporal_predictor") {
                            Some(value) => value,
                            None => bail!("Missing field 'mv_temporal_predictor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#par_denominator: {
                        let field_value = match fields_map.get("par_denominator") {
                            Some(value) => value,
                            None => bail!("Missing field 'par_denominator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#par_numerator: {
                        let field_value = match fields_map.get("par_numerator") {
                            Some(value) => value,
                            None => bail!("Missing field 'par_numerator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#profile: {
                        let field_value = match fields_map.get("profile") {
                            Some(value) => value,
                            None => bail!("Missing field 'profile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#qvbr_quality_level: {
                        let field_value = match fields_map.get("qvbr_quality_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'qvbr_quality_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rate_control_mode: {
                        let field_value = match fields_map.get("rate_control_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'rate_control_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_type: {
                        let field_value = match fields_map.get("scan_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'scan_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scene_change_detect: {
                        let field_value = match fields_map.get("scene_change_detect") {
                            Some(value) => value,
                            None => bail!("Missing field 'scene_change_detect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slices: {
                        let field_value = match fields_map.get("slices") {
                            Some(value) => value,
                            None => bail!("Missing field 'slices' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier: {
                        let field_value = match fields_map.get("tier") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tile_height: {
                        let field_value = match fields_map.get("tile_height") {
                            Some(value) => value,
                            None => bail!("Missing field 'tile_height' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tile_padding: {
                        let field_value = match fields_map.get("tile_padding") {
                            Some(value) => value,
                            None => bail!("Missing field 'tile_padding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tile_width: {
                        let field_value = match fields_map.get("tile_width") {
                            Some(value) => value,
                            None => bail!("Missing field 'tile_width' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timecode_burnin_settings: {
                        let field_value = match fields_map.get("timecode_burnin_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'timecode_burnin_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timecode_insertion: {
                        let field_value = match fields_map.get("timecode_insertion") {
                            Some(value) => value,
                            None => bail!("Missing field 'timecode_insertion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#treeblock_size: {
                        let field_value = match fields_map.get("treeblock_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'treeblock_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
