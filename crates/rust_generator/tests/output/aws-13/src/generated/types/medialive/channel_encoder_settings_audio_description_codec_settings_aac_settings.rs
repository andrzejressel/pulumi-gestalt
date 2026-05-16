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

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("bitrate".to_string(), self.r#bitrate.to_pulumi_value().await);
            map.insert("coding_mode".to_string(), self.r#coding_mode.to_pulumi_value().await);
            map.insert("input_type".to_string(), self.r#input_type.to_pulumi_value().await);
            map.insert("profile".to_string(), self.r#profile.to_pulumi_value().await);
            map.insert("rate_control_mode".to_string(), self.r#rate_control_mode.to_pulumi_value().await);
            map.insert("raw_format".to_string(), self.r#raw_format.to_pulumi_value().await);
            map.insert("sample_rate".to_string(), self.r#sample_rate.to_pulumi_value().await);
            map.insert("spec".to_string(), self.r#spec.to_pulumi_value().await);
            map.insert("vbr_quality".to_string(), self.r#vbr_quality.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#bitrate: {
                        let field_value = match fields_map.get("bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<f64> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#coding_mode: {
                        let field_value = match fields_map.get("coding_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'coding_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#input_type: {
                        let field_value = match fields_map.get("input_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#profile: {
                        let field_value = match fields_map.get("profile") {
                            Some(value) => value,
                            None => bail!("Missing field 'profile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rate_control_mode: {
                        let field_value = match fields_map.get("rate_control_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'rate_control_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#raw_format: {
                        let field_value = match fields_map.get("raw_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'raw_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sample_rate: {
                        let field_value = match fields_map.get("sample_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<f64> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#spec: {
                        let field_value = match fields_map.get("spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#vbr_quality: {
                        let field_value = match fields_map.get("vbr_quality") {
                            Some(value) => value,
                            None => bail!("Missing field 'vbr_quality' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
