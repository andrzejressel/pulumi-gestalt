#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings {
    /// Average bitrate in bits/second.
    #[builder(into)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Option<f64>,
    /// Mono, Stereo, or 5.1 channel layout.
    #[builder(into)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Option<String>,
    /// Set to "broadcasterMixedAd" when input contains pre-mixed main audio + AD (narration) as a stereo pair.
    #[builder(into)]
    #[serde(rename = "inputType")]
    pub r#input_type: Option<String>,
    /// AAC profile.
    #[builder(into)]
    #[serde(rename = "profile")]
    pub r#profile: Option<String>,
    /// The rate control mode.
    #[builder(into)]
    #[serde(rename = "rateControlMode")]
    pub r#rate_control_mode: Option<String>,
    /// Sets LATM/LOAS AAC output for raw containers.
    #[builder(into)]
    #[serde(rename = "rawFormat")]
    pub r#raw_format: Option<String>,
    /// Sample rate in Hz.
    #[builder(into)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Option<f64>,
    /// Use MPEG-2 AAC audio instead of MPEG-4 AAC audio for raw or MPEG-2 Transport Stream containers.
    #[builder(into)]
    #[serde(rename = "spec")]
    pub r#spec: Option<String>,
    /// VBR Quality Level - Only used if rateControlMode is VBR.
    #[builder(into)]
    #[serde(rename = "vbrQuality")]
    pub r#vbr_quality: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "bitrate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bitrate,
                )
                .await,
            );
            map.insert(
                "coding_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#coding_mode,
                )
                .await,
            );
            map.insert(
                "input_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_type,
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
                "rate_control_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rate_control_mode,
                )
                .await,
            );
            map.insert(
                "raw_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#raw_format,
                )
                .await,
            );
            map.insert(
                "sample_rate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sample_rate,
                )
                .await,
            );
            map.insert(
                "spec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spec,
                )
                .await,
            );
            map.insert(
                "vbr_quality".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vbr_quality,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings {
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
                    r#bitrate: {
                        let field_value = match fields_map.get("bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#input_type: {
                        let field_value = match fields_map.get("input_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#rate_control_mode: {
                        let field_value = match fields_map.get("rate_control_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'rate_control_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#raw_format: {
                        let field_value = match fields_map.get("raw_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'raw_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sample_rate: {
                        let field_value = match fields_map.get("sample_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spec: {
                        let field_value = match fields_map.get("spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vbr_quality: {
                        let field_value = match fields_map.get("vbr_quality") {
                            Some(value) => value,
                            None => bail!("Missing field 'vbr_quality' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
