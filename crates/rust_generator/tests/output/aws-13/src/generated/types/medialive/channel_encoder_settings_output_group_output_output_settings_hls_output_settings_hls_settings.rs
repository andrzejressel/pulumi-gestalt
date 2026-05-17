#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings {
    #[builder(into)]
    #[serde(rename = "audioOnlyHlsSettings")]
    pub r#audio_only_hls_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettings>>,
    #[builder(into)]
    #[serde(rename = "fmp4HlsSettings")]
    pub r#fmp_4_hls_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFmp4HlsSettings>>,
    #[builder(into)]
    #[serde(rename = "frameCaptureHlsSettings")]
    pub r#frame_capture_hls_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFrameCaptureHlsSettings>>,
    #[builder(into)]
    #[serde(rename = "standardHlsSettings")]
    pub r#standard_hls_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings {
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
                "audio_only_hls_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audio_only_hls_settings,
                )
                .await,
            );
            map.insert(
                "fmp_4_hls_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fmp_4_hls_settings,
                )
                .await,
            );
            map.insert(
                "frame_capture_hls_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#frame_capture_hls_settings,
                )
                .await,
            );
            map.insert(
                "standard_hls_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#standard_hls_settings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings {
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
                    r#audio_only_hls_settings: {
                        let field_value = match fields_map.get("audio_only_hls_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_only_hls_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fmp_4_hls_settings: {
                        let field_value = match fields_map.get("fmp_4_hls_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'fmp_4_hls_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frame_capture_hls_settings: {
                        let field_value = match fields_map.get("frame_capture_hls_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'frame_capture_hls_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#standard_hls_settings: {
                        let field_value = match fields_map.get("standard_hls_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'standard_hls_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
