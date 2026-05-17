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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings {
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
                    "bitrate",
                    &self.r#bitrate,
                ),
                to_pulumi_object_field(
                    "buf_fill_pct",
                    &self.r#buf_fill_pct,
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
                    "entropy_encoding",
                    &self.r#entropy_encoding,
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
                    "force_field_pictures",
                    &self.r#force_field_pictures,
                ),
                to_pulumi_object_field(
                    "framerate_control",
                    &self.r#framerate_control,
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
                    "gop_b_reference",
                    &self.r#gop_b_reference,
                ),
                to_pulumi_object_field(
                    "gop_closed_cadence",
                    &self.r#gop_closed_cadence,
                ),
                to_pulumi_object_field(
                    "gop_num_b_frames",
                    &self.r#gop_num_b_frames,
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
                    "num_ref_frames",
                    &self.r#num_ref_frames,
                ),
                to_pulumi_object_field(
                    "par_control",
                    &self.r#par_control,
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
                    "quality_level",
                    &self.r#quality_level,
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
                    "softness",
                    &self.r#softness,
                ),
                to_pulumi_object_field(
                    "spatial_aq",
                    &self.r#spatial_aq,
                ),
                to_pulumi_object_field(
                    "subgop_length",
                    &self.r#subgop_length,
                ),
                to_pulumi_object_field(
                    "syntax",
                    &self.r#syntax,
                ),
                to_pulumi_object_field(
                    "temporal_aq",
                    &self.r#temporal_aq,
                ),
                to_pulumi_object_field(
                    "timecode_insertion",
                    &self.r#timecode_insertion,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings {
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
                    r#bitrate: {
                        let field_value = match fields_map.get("bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#buf_fill_pct: {
                        let field_value = match fields_map.get("buf_fill_pct") {
                            Some(value) => value,
                            None => bail!("Missing field 'buf_fill_pct' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#entropy_encoding: {
                        let field_value = match fields_map.get("entropy_encoding") {
                            Some(value) => value,
                            None => bail!("Missing field 'entropy_encoding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#force_field_pictures: {
                        let field_value = match fields_map.get("force_field_pictures") {
                            Some(value) => value,
                            None => bail!("Missing field 'force_field_pictures' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#framerate_control: {
                        let field_value = match fields_map.get("framerate_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'framerate_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#gop_b_reference: {
                        let field_value = match fields_map.get("gop_b_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'gop_b_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#gop_num_b_frames: {
                        let field_value = match fields_map.get("gop_num_b_frames") {
                            Some(value) => value,
                            None => bail!("Missing field 'gop_num_b_frames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#num_ref_frames: {
                        let field_value = match fields_map.get("num_ref_frames") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_ref_frames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#par_control: {
                        let field_value = match fields_map.get("par_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'par_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#quality_level: {
                        let field_value = match fields_map.get("quality_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'quality_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#softness: {
                        let field_value = match fields_map.get("softness") {
                            Some(value) => value,
                            None => bail!("Missing field 'softness' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spatial_aq: {
                        let field_value = match fields_map.get("spatial_aq") {
                            Some(value) => value,
                            None => bail!("Missing field 'spatial_aq' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subgop_length: {
                        let field_value = match fields_map.get("subgop_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'subgop_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#syntax: {
                        let field_value = match fields_map.get("syntax") {
                            Some(value) => value,
                            None => bail!("Missing field 'syntax' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#temporal_aq: {
                        let field_value = match fields_map.get("temporal_aq") {
                            Some(value) => value,
                            None => bail!("Missing field 'temporal_aq' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
