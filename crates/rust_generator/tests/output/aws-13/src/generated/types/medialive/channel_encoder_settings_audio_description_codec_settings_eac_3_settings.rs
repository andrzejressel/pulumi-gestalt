#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings {
    /// Sets the attenuation control.
    #[builder(into)]
    #[serde(rename = "attenuationControl")]
    pub r#attenuation_control: Option<String>,
    /// Average bitrate in bits/second.
    #[builder(into)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Option<f64>,
    /// Specifies the bitstream mode (bsmod) for the emitted AC-3 stream.
    #[builder(into)]
    #[serde(rename = "bitstreamMode")]
    pub r#bitstream_mode: Option<String>,
    /// Dolby Digital Plus coding mode.
    #[builder(into)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Option<String>,
    #[builder(into)]
    #[serde(rename = "dcFilter")]
    pub r#dc_filter: Option<String>,
    #[builder(into)]
    #[serde(rename = "dialnorm")]
    pub r#dialnorm: Option<i32>,
    /// Sets the Dolby dynamic range compression profile.
    #[builder(into)]
    #[serde(rename = "drcLine")]
    pub r#drc_line: Option<String>,
    /// Sets the profile for heavy Dolby dynamic range compression.
    #[builder(into)]
    #[serde(rename = "drcRf")]
    pub r#drc_rf: Option<String>,
    #[builder(into)]
    #[serde(rename = "lfeControl")]
    pub r#lfe_control: Option<String>,
    /// When set to enabled, applies a 120Hz lowpass filter to the LFE channel prior to encoding.
    #[builder(into)]
    #[serde(rename = "lfeFilter")]
    pub r#lfe_filter: Option<String>,
    #[builder(into)]
    #[serde(rename = "loRoCenterMixLevel")]
    pub r#lo_ro_center_mix_level: Option<f64>,
    #[builder(into)]
    #[serde(rename = "loRoSurroundMixLevel")]
    pub r#lo_ro_surround_mix_level: Option<f64>,
    #[builder(into)]
    #[serde(rename = "ltRtCenterMixLevel")]
    pub r#lt_rt_center_mix_level: Option<f64>,
    #[builder(into)]
    #[serde(rename = "ltRtSurroundMixLevel")]
    pub r#lt_rt_surround_mix_level: Option<f64>,
    /// Metadata control.
    #[builder(into)]
    #[serde(rename = "metadataControl")]
    pub r#metadata_control: Option<String>,
    #[builder(into)]
    #[serde(rename = "passthroughControl")]
    pub r#passthrough_control: Option<String>,
    #[builder(into)]
    #[serde(rename = "phaseControl")]
    pub r#phase_control: Option<String>,
    #[builder(into)]
    #[serde(rename = "stereoDownmix")]
    pub r#stereo_downmix: Option<String>,
    #[builder(into)]
    #[serde(rename = "surroundExMode")]
    pub r#surround_ex_mode: Option<String>,
    #[builder(into)]
    #[serde(rename = "surroundMode")]
    pub r#surround_mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings {
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
                    "attenuation_control",
                    &self.r#attenuation_control,
                ),
                to_pulumi_object_field(
                    "bitrate",
                    &self.r#bitrate,
                ),
                to_pulumi_object_field(
                    "bitstream_mode",
                    &self.r#bitstream_mode,
                ),
                to_pulumi_object_field(
                    "coding_mode",
                    &self.r#coding_mode,
                ),
                to_pulumi_object_field(
                    "dc_filter",
                    &self.r#dc_filter,
                ),
                to_pulumi_object_field(
                    "dialnorm",
                    &self.r#dialnorm,
                ),
                to_pulumi_object_field(
                    "drc_line",
                    &self.r#drc_line,
                ),
                to_pulumi_object_field(
                    "drc_rf",
                    &self.r#drc_rf,
                ),
                to_pulumi_object_field(
                    "lfe_control",
                    &self.r#lfe_control,
                ),
                to_pulumi_object_field(
                    "lfe_filter",
                    &self.r#lfe_filter,
                ),
                to_pulumi_object_field(
                    "lo_ro_center_mix_level",
                    &self.r#lo_ro_center_mix_level,
                ),
                to_pulumi_object_field(
                    "lo_ro_surround_mix_level",
                    &self.r#lo_ro_surround_mix_level,
                ),
                to_pulumi_object_field(
                    "lt_rt_center_mix_level",
                    &self.r#lt_rt_center_mix_level,
                ),
                to_pulumi_object_field(
                    "lt_rt_surround_mix_level",
                    &self.r#lt_rt_surround_mix_level,
                ),
                to_pulumi_object_field(
                    "metadata_control",
                    &self.r#metadata_control,
                ),
                to_pulumi_object_field(
                    "passthrough_control",
                    &self.r#passthrough_control,
                ),
                to_pulumi_object_field(
                    "phase_control",
                    &self.r#phase_control,
                ),
                to_pulumi_object_field(
                    "stereo_downmix",
                    &self.r#stereo_downmix,
                ),
                to_pulumi_object_field(
                    "surround_ex_mode",
                    &self.r#surround_ex_mode,
                ),
                to_pulumi_object_field(
                    "surround_mode",
                    &self.r#surround_mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings {
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
                    r#attenuation_control: {
                        let field_value = match fields_map.get("attenuation_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'attenuation_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#bitstream_mode: {
                        let field_value = match fields_map.get("bitstream_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitstream_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#coding_mode: {
                        let field_value = match fields_map.get("coding_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'coding_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dc_filter: {
                        let field_value = match fields_map.get("dc_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'dc_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dialnorm: {
                        let field_value = match fields_map.get("dialnorm") {
                            Some(value) => value,
                            None => bail!("Missing field 'dialnorm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#drc_line: {
                        let field_value = match fields_map.get("drc_line") {
                            Some(value) => value,
                            None => bail!("Missing field 'drc_line' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#drc_rf: {
                        let field_value = match fields_map.get("drc_rf") {
                            Some(value) => value,
                            None => bail!("Missing field 'drc_rf' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lfe_control: {
                        let field_value = match fields_map.get("lfe_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'lfe_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lfe_filter: {
                        let field_value = match fields_map.get("lfe_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'lfe_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lo_ro_center_mix_level: {
                        let field_value = match fields_map.get("lo_ro_center_mix_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'lo_ro_center_mix_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lo_ro_surround_mix_level: {
                        let field_value = match fields_map.get("lo_ro_surround_mix_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'lo_ro_surround_mix_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lt_rt_center_mix_level: {
                        let field_value = match fields_map.get("lt_rt_center_mix_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'lt_rt_center_mix_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lt_rt_surround_mix_level: {
                        let field_value = match fields_map.get("lt_rt_surround_mix_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'lt_rt_surround_mix_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata_control: {
                        let field_value = match fields_map.get("metadata_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#passthrough_control: {
                        let field_value = match fields_map.get("passthrough_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'passthrough_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#phase_control: {
                        let field_value = match fields_map.get("phase_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'phase_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stereo_downmix: {
                        let field_value = match fields_map.get("stereo_downmix") {
                            Some(value) => value,
                            None => bail!("Missing field 'stereo_downmix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#surround_ex_mode: {
                        let field_value = match fields_map.get("surround_ex_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'surround_ex_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#surround_mode: {
                        let field_value = match fields_map.get("surround_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'surround_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
