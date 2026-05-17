#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettings {
    /// Aac Settings. See AAC Settings for more details.
    #[builder(into)]
    #[serde(rename = "aacSettings")]
    pub r#aac_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings>>,
    /// Ac3 Settings. See AC3 Settings for more details.
    #[builder(into)]
    #[serde(rename = "ac3Settings")]
    pub r#ac_3_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings>>,
    /// Eac3 Atmos Settings. See EAC3 Atmos Settings
    #[builder(into)]
    #[serde(rename = "eac3AtmosSettings")]
    pub r#eac_3_atmos_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings>>,
    /// Eac3 Settings. See EAC3 Settings
    #[builder(into)]
    #[serde(rename = "eac3Settings")]
    pub r#eac_3_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings>>,
    #[builder(into)]
    #[serde(rename = "mp2Settings")]
    pub r#mp_2_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsMp2Settings>>,
    #[builder(into)]
    #[serde(rename = "passThroughSettings")]
    pub r#pass_through_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsPassThroughSettings>>,
    #[builder(into)]
    #[serde(rename = "wavSettings")]
    pub r#wav_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsWavSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsAudioDescriptionCodecSettings {
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
                "aac_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aac_settings,
                )
                .await,
            );
            map.insert(
                "ac_3_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ac_3_settings,
                )
                .await,
            );
            map.insert(
                "eac_3_atmos_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#eac_3_atmos_settings,
                )
                .await,
            );
            map.insert(
                "eac_3_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#eac_3_settings,
                )
                .await,
            );
            map.insert(
                "mp_2_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mp_2_settings,
                )
                .await,
            );
            map.insert(
                "pass_through_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pass_through_settings,
                )
                .await,
            );
            map.insert(
                "wav_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#wav_settings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsAudioDescriptionCodecSettings {
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
                    r#aac_settings: {
                        let field_value = match fields_map.get("aac_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'aac_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ac_3_settings: {
                        let field_value = match fields_map.get("ac_3_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'ac_3_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eac_3_atmos_settings: {
                        let field_value = match fields_map.get("eac_3_atmos_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'eac_3_atmos_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eac_3_settings: {
                        let field_value = match fields_map.get("eac_3_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'eac_3_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mp_2_settings: {
                        let field_value = match fields_map.get("mp_2_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'mp_2_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pass_through_settings: {
                        let field_value = match fields_map.get("pass_through_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'pass_through_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wav_settings: {
                        let field_value = match fields_map.get("wav_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'wav_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
