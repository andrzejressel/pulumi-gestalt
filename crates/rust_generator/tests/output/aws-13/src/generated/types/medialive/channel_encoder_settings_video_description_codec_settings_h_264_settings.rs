#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings {
    /// Enables or disables adaptive quantization.
    #[builder(into)]
    pub r#adaptive_quantization: Option<String>,
    /// Indicates that AFD values will be written into the output stream.
    #[builder(into)]
    pub r#afd_signaling: Option<String>,
    /// Average bitrate in bits/second.
    #[builder(into)]
    pub r#bitrate: Option<i32>,
    #[builder(into)]
    pub r#buf_fill_pct: Option<i32>,
    /// Size of buffer in bits.
    #[builder(into)]
    pub r#buf_size: Option<i32>,
    /// Includes color space metadata in the output.
    #[builder(into)]
    pub r#color_metadata: Option<String>,
    /// Entropy encoding mode.
    #[builder(into)]
    pub r#entropy_encoding: Option<String>,
    /// Filters to apply to an encode. See H264 Filter Settings for more details.
    #[builder(into)]
    pub r#filter_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettings>>,
    /// Four bit AFD value to write on all frames of video in the output stream.
    #[builder(into)]
    pub r#fixed_afd: Option<String>,
    #[builder(into)]
    pub r#flicker_aq: Option<String>,
    /// Controls whether coding is performed on a field basis or on a frame basis.
    #[builder(into)]
    pub r#force_field_pictures: Option<String>,
    /// Indicates how the output video frame rate is specified.
    #[builder(into)]
    pub r#framerate_control: Option<String>,
    /// Framerate denominator.
    #[builder(into)]
    pub r#framerate_denominator: Option<i32>,
    /// Framerate numerator.
    #[builder(into)]
    pub r#framerate_numerator: Option<i32>,
    /// GOP-B reference.
    #[builder(into)]
    pub r#gop_b_reference: Option<String>,
    /// Frequency of closed GOPs.
    #[builder(into)]
    pub r#gop_closed_cadence: Option<i32>,
    /// Number of B-frames between reference frames.
    #[builder(into)]
    pub r#gop_num_b_frames: Option<i32>,
    /// GOP size in units of either frames of seconds per `gop_size_units`.
    #[builder(into)]
    pub r#gop_size: Option<f64>,
    /// Indicates if the `gop_size` is specified in frames or seconds.
    #[builder(into)]
    pub r#gop_size_units: Option<String>,
    /// H264 level.
    #[builder(into)]
    pub r#level: Option<String>,
    /// Amount of lookahead.
    #[builder(into)]
    pub r#look_ahead_rate_control: Option<String>,
    /// Set the maximum bitrate in order to accommodate expected spikes in the complexity of the video.
    #[builder(into)]
    pub r#max_bitrate: Option<i32>,
    /// Min interval.
    #[builder(into)]
    pub r#min_i_interval: Option<i32>,
    /// Number of reference frames to use.
    #[builder(into)]
    pub r#num_ref_frames: Option<i32>,
    /// Indicates how the output pixel aspect ratio is specified.
    #[builder(into)]
    pub r#par_control: Option<String>,
    /// Pixel Aspect Ratio denominator.
    #[builder(into)]
    pub r#par_denominator: Option<i32>,
    /// Pixel Aspect Ratio numerator.
    #[builder(into)]
    pub r#par_numerator: Option<i32>,
    /// H264 profile.
    #[builder(into)]
    pub r#profile: Option<String>,
    /// Quality level.
    #[builder(into)]
    pub r#quality_level: Option<String>,
    /// Controls the target quality for the video encode.
    #[builder(into)]
    pub r#qvbr_quality_level: Option<i32>,
    /// Rate control mode.
    #[builder(into)]
    pub r#rate_control_mode: Option<String>,
    /// Sets the scan type of the output.
    #[builder(into)]
    pub r#scan_type: Option<String>,
    /// Scene change detection.
    #[builder(into)]
    pub r#scene_change_detect: Option<String>,
    /// Number of slices per picture.
    #[builder(into)]
    pub r#slices: Option<i32>,
    /// Softness.
    #[builder(into)]
    pub r#softness: Option<i32>,
    /// Makes adjustments within each frame based on spatial variation of content complexity.
    #[builder(into)]
    pub r#spatial_aq: Option<String>,
    /// Subgop length.
    #[builder(into)]
    pub r#subgop_length: Option<String>,
    /// Produces a bitstream compliant with SMPTE RP-2027.
    #[builder(into)]
    pub r#syntax: Option<String>,
    /// Makes adjustments within each frame based on temporal variation of content complexity.
    #[builder(into)]
    pub r#temporal_aq: Option<String>,
    /// Determines how timecodes should be inserted into the video elementary stream.
    #[builder(into)]
    pub r#timecode_insertion: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings {
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
                    "bitrate",
                    &self.r#bitrate,
                ),
                to_pulumi_object_field(
                    "bufFillPct",
                    &self.r#buf_fill_pct,
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
                    "entropyEncoding",
                    &self.r#entropy_encoding,
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
                    "forceFieldPictures",
                    &self.r#force_field_pictures,
                ),
                to_pulumi_object_field(
                    "framerateControl",
                    &self.r#framerate_control,
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
                    "gopBReference",
                    &self.r#gop_b_reference,
                ),
                to_pulumi_object_field(
                    "gopClosedCadence",
                    &self.r#gop_closed_cadence,
                ),
                to_pulumi_object_field(
                    "gopNumBFrames",
                    &self.r#gop_num_b_frames,
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
                    "numRefFrames",
                    &self.r#num_ref_frames,
                ),
                to_pulumi_object_field(
                    "parControl",
                    &self.r#par_control,
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
                    "qualityLevel",
                    &self.r#quality_level,
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
                    "softness",
                    &self.r#softness,
                ),
                to_pulumi_object_field(
                    "spatialAq",
                    &self.r#spatial_aq,
                ),
                to_pulumi_object_field(
                    "subgopLength",
                    &self.r#subgop_length,
                ),
                to_pulumi_object_field(
                    "syntax",
                    &self.r#syntax,
                ),
                to_pulumi_object_field(
                    "temporalAq",
                    &self.r#temporal_aq,
                ),
                to_pulumi_object_field(
                    "timecodeInsertion",
                    &self.r#timecode_insertion,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                    r#bitrate: {
                        let field_value = match fields_map.get("bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#buf_fill_pct: {
                        let field_value = match fields_map.get("bufFillPct") {
                            Some(value) => value,
                            None => bail!("Missing field 'bufFillPct' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#entropy_encoding: {
                        let field_value = match fields_map.get("entropyEncoding") {
                            Some(value) => value,
                            None => bail!("Missing field 'entropyEncoding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#force_field_pictures: {
                        let field_value = match fields_map.get("forceFieldPictures") {
                            Some(value) => value,
                            None => bail!("Missing field 'forceFieldPictures' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#framerate_control: {
                        let field_value = match fields_map.get("framerateControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'framerateControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#gop_b_reference: {
                        let field_value = match fields_map.get("gopBReference") {
                            Some(value) => value,
                            None => bail!("Missing field 'gopBReference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#gop_num_b_frames: {
                        let field_value = match fields_map.get("gopNumBFrames") {
                            Some(value) => value,
                            None => bail!("Missing field 'gopNumBFrames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#num_ref_frames: {
                        let field_value = match fields_map.get("numRefFrames") {
                            Some(value) => value,
                            None => bail!("Missing field 'numRefFrames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#par_control: {
                        let field_value = match fields_map.get("parControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'parControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#quality_level: {
                        let field_value = match fields_map.get("qualityLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'qualityLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#softness: {
                        let field_value = match fields_map.get("softness") {
                            Some(value) => value,
                            None => bail!("Missing field 'softness' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spatial_aq: {
                        let field_value = match fields_map.get("spatialAq") {
                            Some(value) => value,
                            None => bail!("Missing field 'spatialAq' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subgop_length: {
                        let field_value = match fields_map.get("subgopLength") {
                            Some(value) => value,
                            None => bail!("Missing field 'subgopLength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("temporalAq") {
                            Some(value) => value,
                            None => bail!("Missing field 'temporalAq' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
