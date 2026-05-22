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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "adaptiveQuantization",
                    &self.r#adaptive_quantization,
                ),
                to_pulumi_object_field(
                    "afdSignaling",
                    &self.r#afd_signaling,
                ),
                to_pulumi_object_field(
                    "alternativeTransferFunction",
                    &self.r#alternative_transfer_function,
                ),
                to_pulumi_object_field(
                    "bitrate",
                    &self.r#bitrate,
                ),
                to_pulumi_object_field(
                    "bufSize",
                    &self.r#buf_size,
                ),
                to_pulumi_object_field(
                    "colorMetadata",
                    &self.r#color_metadata,
                ),
                to_pulumi_object_field(
                    "colorSpaceSettings",
                    &self.r#color_space_settings,
                ),
                to_pulumi_object_field(
                    "filterSettings",
                    &self.r#filter_settings,
                ),
                to_pulumi_object_field(
                    "fixedAfd",
                    &self.r#fixed_afd,
                ),
                to_pulumi_object_field(
                    "flickerAq",
                    &self.r#flicker_aq,
                ),
                to_pulumi_object_field(
                    "framerateDenominator",
                    &self.r#framerate_denominator,
                ),
                to_pulumi_object_field(
                    "framerateNumerator",
                    &self.r#framerate_numerator,
                ),
                to_pulumi_object_field(
                    "gopClosedCadence",
                    &self.r#gop_closed_cadence,
                ),
                to_pulumi_object_field(
                    "gopSize",
                    &self.r#gop_size,
                ),
                to_pulumi_object_field(
                    "gopSizeUnits",
                    &self.r#gop_size_units,
                ),
                to_pulumi_object_field(
                    "level",
                    &self.r#level,
                ),
                to_pulumi_object_field(
                    "lookAheadRateControl",
                    &self.r#look_ahead_rate_control,
                ),
                to_pulumi_object_field(
                    "maxBitrate",
                    &self.r#max_bitrate,
                ),
                to_pulumi_object_field(
                    "minIInterval",
                    &self.r#min_i_interval,
                ),
                to_pulumi_object_field(
                    "minQp",
                    &self.r#min_qp,
                ),
                to_pulumi_object_field(
                    "mvOverPictureBoundaries",
                    &self.r#mv_over_picture_boundaries,
                ),
                to_pulumi_object_field(
                    "mvTemporalPredictor",
                    &self.r#mv_temporal_predictor,
                ),
                to_pulumi_object_field(
                    "parDenominator",
                    &self.r#par_denominator,
                ),
                to_pulumi_object_field(
                    "parNumerator",
                    &self.r#par_numerator,
                ),
                to_pulumi_object_field(
                    "profile",
                    &self.r#profile,
                ),
                to_pulumi_object_field(
                    "qvbrQualityLevel",
                    &self.r#qvbr_quality_level,
                ),
                to_pulumi_object_field(
                    "rateControlMode",
                    &self.r#rate_control_mode,
                ),
                to_pulumi_object_field(
                    "scanType",
                    &self.r#scan_type,
                ),
                to_pulumi_object_field(
                    "sceneChangeDetect",
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
                    "tileHeight",
                    &self.r#tile_height,
                ),
                to_pulumi_object_field(
                    "tilePadding",
                    &self.r#tile_padding,
                ),
                to_pulumi_object_field(
                    "tileWidth",
                    &self.r#tile_width,
                ),
                to_pulumi_object_field(
                    "timecodeBurninSettings",
                    &self.r#timecode_burnin_settings,
                ),
                to_pulumi_object_field(
                    "timecodeInsertion",
                    &self.r#timecode_insertion,
                ),
                to_pulumi_object_field(
                    "treeblockSize",
                    &self.r#treeblock_size,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("adaptiveQuantization") {
                            Some(value) => value,
                            None => bail!("Missing field 'adaptiveQuantization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#afd_signaling: {
                        let field_value = match fields_map.get("afdSignaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'afdSignaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#alternative_transfer_function: {
                        let field_value = match fields_map.get("alternativeTransferFunction") {
                            Some(value) => value,
                            None => bail!("Missing field 'alternativeTransferFunction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("bufSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'bufSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#color_metadata: {
                        let field_value = match fields_map.get("colorMetadata") {
                            Some(value) => value,
                            None => bail!("Missing field 'colorMetadata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#color_space_settings: {
                        let field_value = match fields_map.get("colorSpaceSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'colorSpaceSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_settings: {
                        let field_value = match fields_map.get("filterSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'filterSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fixed_afd: {
                        let field_value = match fields_map.get("fixedAfd") {
                            Some(value) => value,
                            None => bail!("Missing field 'fixedAfd' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flicker_aq: {
                        let field_value = match fields_map.get("flickerAq") {
                            Some(value) => value,
                            None => bail!("Missing field 'flickerAq' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#framerate_denominator: {
                        let field_value = match fields_map.get("framerateDenominator") {
                            Some(value) => value,
                            None => bail!("Missing field 'framerateDenominator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#framerate_numerator: {
                        let field_value = match fields_map.get("framerateNumerator") {
                            Some(value) => value,
                            None => bail!("Missing field 'framerateNumerator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gop_closed_cadence: {
                        let field_value = match fields_map.get("gopClosedCadence") {
                            Some(value) => value,
                            None => bail!("Missing field 'gopClosedCadence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gop_size: {
                        let field_value = match fields_map.get("gopSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'gopSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gop_size_units: {
                        let field_value = match fields_map.get("gopSizeUnits") {
                            Some(value) => value,
                            None => bail!("Missing field 'gopSizeUnits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("lookAheadRateControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'lookAheadRateControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_bitrate: {
                        let field_value = match fields_map.get("maxBitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxBitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_i_interval: {
                        let field_value = match fields_map.get("minIInterval") {
                            Some(value) => value,
                            None => bail!("Missing field 'minIInterval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_qp: {
                        let field_value = match fields_map.get("minQp") {
                            Some(value) => value,
                            None => bail!("Missing field 'minQp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mv_over_picture_boundaries: {
                        let field_value = match fields_map.get("mvOverPictureBoundaries") {
                            Some(value) => value,
                            None => bail!("Missing field 'mvOverPictureBoundaries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mv_temporal_predictor: {
                        let field_value = match fields_map.get("mvTemporalPredictor") {
                            Some(value) => value,
                            None => bail!("Missing field 'mvTemporalPredictor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#par_denominator: {
                        let field_value = match fields_map.get("parDenominator") {
                            Some(value) => value,
                            None => bail!("Missing field 'parDenominator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#par_numerator: {
                        let field_value = match fields_map.get("parNumerator") {
                            Some(value) => value,
                            None => bail!("Missing field 'parNumerator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("qvbrQualityLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'qvbrQualityLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rate_control_mode: {
                        let field_value = match fields_map.get("rateControlMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'rateControlMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_type: {
                        let field_value = match fields_map.get("scanType") {
                            Some(value) => value,
                            None => bail!("Missing field 'scanType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scene_change_detect: {
                        let field_value = match fields_map.get("sceneChangeDetect") {
                            Some(value) => value,
                            None => bail!("Missing field 'sceneChangeDetect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("tileHeight") {
                            Some(value) => value,
                            None => bail!("Missing field 'tileHeight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tile_padding: {
                        let field_value = match fields_map.get("tilePadding") {
                            Some(value) => value,
                            None => bail!("Missing field 'tilePadding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tile_width: {
                        let field_value = match fields_map.get("tileWidth") {
                            Some(value) => value,
                            None => bail!("Missing field 'tileWidth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timecode_burnin_settings: {
                        let field_value = match fields_map.get("timecodeBurninSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'timecodeBurninSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timecode_insertion: {
                        let field_value = match fields_map.get("timecodeInsertion") {
                            Some(value) => value,
                            None => bail!("Missing field 'timecodeInsertion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#treeblock_size: {
                        let field_value = match fields_map.get("treeblockSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'treeblockSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
