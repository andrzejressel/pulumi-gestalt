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
                    "attenuationControl",
                    &self.r#attenuation_control,
                ),
                to_pulumi_object_field(
                    "bitrate",
                    &self.r#bitrate,
                ),
                to_pulumi_object_field(
                    "bitstreamMode",
                    &self.r#bitstream_mode,
                ),
                to_pulumi_object_field(
                    "codingMode",
                    &self.r#coding_mode,
                ),
                to_pulumi_object_field(
                    "dcFilter",
                    &self.r#dc_filter,
                ),
                to_pulumi_object_field(
                    "dialnorm",
                    &self.r#dialnorm,
                ),
                to_pulumi_object_field(
                    "drcLine",
                    &self.r#drc_line,
                ),
                to_pulumi_object_field(
                    "drcRf",
                    &self.r#drc_rf,
                ),
                to_pulumi_object_field(
                    "lfeControl",
                    &self.r#lfe_control,
                ),
                to_pulumi_object_field(
                    "lfeFilter",
                    &self.r#lfe_filter,
                ),
                to_pulumi_object_field(
                    "loRoCenterMixLevel",
                    &self.r#lo_ro_center_mix_level,
                ),
                to_pulumi_object_field(
                    "loRoSurroundMixLevel",
                    &self.r#lo_ro_surround_mix_level,
                ),
                to_pulumi_object_field(
                    "ltRtCenterMixLevel",
                    &self.r#lt_rt_center_mix_level,
                ),
                to_pulumi_object_field(
                    "ltRtSurroundMixLevel",
                    &self.r#lt_rt_surround_mix_level,
                ),
                to_pulumi_object_field(
                    "metadataControl",
                    &self.r#metadata_control,
                ),
                to_pulumi_object_field(
                    "passthroughControl",
                    &self.r#passthrough_control,
                ),
                to_pulumi_object_field(
                    "phaseControl",
                    &self.r#phase_control,
                ),
                to_pulumi_object_field(
                    "stereoDownmix",
                    &self.r#stereo_downmix,
                ),
                to_pulumi_object_field(
                    "surroundExMode",
                    &self.r#surround_ex_mode,
                ),
                to_pulumi_object_field(
                    "surroundMode",
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
                        let field_value = match fields_map.get("attenuationControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'attenuationControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("bitstreamMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitstreamMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#coding_mode: {
                        let field_value = match fields_map.get("codingMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'codingMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dc_filter: {
                        let field_value = match fields_map.get("dcFilter") {
                            Some(value) => value,
                            None => bail!("Missing field 'dcFilter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("drcLine") {
                            Some(value) => value,
                            None => bail!("Missing field 'drcLine' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#drc_rf: {
                        let field_value = match fields_map.get("drcRf") {
                            Some(value) => value,
                            None => bail!("Missing field 'drcRf' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lfe_control: {
                        let field_value = match fields_map.get("lfeControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'lfeControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lfe_filter: {
                        let field_value = match fields_map.get("lfeFilter") {
                            Some(value) => value,
                            None => bail!("Missing field 'lfeFilter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lo_ro_center_mix_level: {
                        let field_value = match fields_map.get("loRoCenterMixLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'loRoCenterMixLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lo_ro_surround_mix_level: {
                        let field_value = match fields_map.get("loRoSurroundMixLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'loRoSurroundMixLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lt_rt_center_mix_level: {
                        let field_value = match fields_map.get("ltRtCenterMixLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'ltRtCenterMixLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lt_rt_surround_mix_level: {
                        let field_value = match fields_map.get("ltRtSurroundMixLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'ltRtSurroundMixLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata_control: {
                        let field_value = match fields_map.get("metadataControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadataControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#passthrough_control: {
                        let field_value = match fields_map.get("passthroughControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'passthroughControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#phase_control: {
                        let field_value = match fields_map.get("phaseControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'phaseControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stereo_downmix: {
                        let field_value = match fields_map.get("stereoDownmix") {
                            Some(value) => value,
                            None => bail!("Missing field 'stereoDownmix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#surround_ex_mode: {
                        let field_value = match fields_map.get("surroundExMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'surroundExMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#surround_mode: {
                        let field_value = match fields_map.get("surroundMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'surroundMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
