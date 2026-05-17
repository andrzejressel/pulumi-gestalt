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

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "adaptive_quantization".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#adaptive_quantization,
                )
                .await,
            );
            map.insert(
                "afd_signaling".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#afd_signaling,
                )
                .await,
            );
            map.insert(
                "bitrate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bitrate,
                )
                .await,
            );
            map.insert(
                "buf_fill_pct".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#buf_fill_pct,
                )
                .await,
            );
            map.insert(
                "buf_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#buf_size,
                )
                .await,
            );
            map.insert(
                "color_metadata".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#color_metadata,
                )
                .await,
            );
            map.insert(
                "entropy_encoding".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#entropy_encoding,
                )
                .await,
            );
            map.insert(
                "filter_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filter_settings,
                )
                .await,
            );
            map.insert(
                "fixed_afd".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fixed_afd,
                )
                .await,
            );
            map.insert(
                "flicker_aq".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#flicker_aq,
                )
                .await,
            );
            map.insert(
                "force_field_pictures".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#force_field_pictures,
                )
                .await,
            );
            map.insert(
                "framerate_control".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#framerate_control,
                )
                .await,
            );
            map.insert(
                "framerate_denominator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#framerate_denominator,
                )
                .await,
            );
            map.insert(
                "framerate_numerator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#framerate_numerator,
                )
                .await,
            );
            map.insert(
                "gop_b_reference".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gop_b_reference,
                )
                .await,
            );
            map.insert(
                "gop_closed_cadence".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gop_closed_cadence,
                )
                .await,
            );
            map.insert(
                "gop_num_b_frames".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gop_num_b_frames,
                )
                .await,
            );
            map.insert(
                "gop_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gop_size,
                )
                .await,
            );
            map.insert(
                "gop_size_units".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gop_size_units,
                )
                .await,
            );
            map.insert(
                "level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#level,
                )
                .await,
            );
            map.insert(
                "look_ahead_rate_control".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#look_ahead_rate_control,
                )
                .await,
            );
            map.insert(
                "max_bitrate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_bitrate,
                )
                .await,
            );
            map.insert(
                "min_i_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_i_interval,
                )
                .await,
            );
            map.insert(
                "num_ref_frames".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#num_ref_frames,
                )
                .await,
            );
            map.insert(
                "par_control".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#par_control,
                )
                .await,
            );
            map.insert(
                "par_denominator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#par_denominator,
                )
                .await,
            );
            map.insert(
                "par_numerator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#par_numerator,
                )
                .await,
            );
            map.insert(
                "profile".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#profile,
                )
                .await,
            );
            map.insert(
                "quality_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#quality_level,
                )
                .await,
            );
            map.insert(
                "qvbr_quality_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#qvbr_quality_level,
                )
                .await,
            );
            map.insert(
                "rate_control_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rate_control_mode,
                )
                .await,
            );
            map.insert(
                "scan_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scan_type,
                )
                .await,
            );
            map.insert(
                "scene_change_detect".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scene_change_detect,
                )
                .await,
            );
            map.insert(
                "slices".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#slices,
                )
                .await,
            );
            map.insert(
                "softness".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#softness,
                )
                .await,
            );
            map.insert(
                "spatial_aq".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spatial_aq,
                )
                .await,
            );
            map.insert(
                "subgop_length".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subgop_length,
                )
                .await,
            );
            map.insert(
                "syntax".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#syntax,
                )
                .await,
            );
            map.insert(
                "temporal_aq".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#temporal_aq,
                )
                .await,
            );
            map.insert(
                "timecode_insertion".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timecode_insertion,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
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
