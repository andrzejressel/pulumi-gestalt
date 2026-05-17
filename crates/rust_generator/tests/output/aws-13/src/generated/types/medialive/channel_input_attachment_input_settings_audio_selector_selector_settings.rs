#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettings {
    /// Audio HLS Rendition Selection. See Audio HLS Rendition Selection for more details.
    #[builder(into)]
    #[serde(rename = "audioHlsRenditionSelection")]
    pub r#audio_hls_rendition_selection: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioHlsRenditionSelection>>,
    /// Audio Language Selection. See Audio Language Selection for more details.
    #[builder(into)]
    #[serde(rename = "audioLanguageSelection")]
    pub r#audio_language_selection: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioLanguageSelection>>,
    /// Audio Pid Selection. See Audio PID Selection for more details.
    #[builder(into)]
    #[serde(rename = "audioPidSelection")]
    pub r#audio_pid_selection: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioPidSelection>>,
    /// Audio Track Selection. See Audio Track Selection for more details.
    #[builder(into)]
    #[serde(rename = "audioTrackSelection")]
    pub r#audio_track_selection: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelection>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettings {
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
                "audio_hls_rendition_selection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audio_hls_rendition_selection,
                )
                .await,
            );
            map.insert(
                "audio_language_selection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audio_language_selection,
                )
                .await,
            );
            map.insert(
                "audio_pid_selection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audio_pid_selection,
                )
                .await,
            );
            map.insert(
                "audio_track_selection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audio_track_selection,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettings {
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
                    r#audio_hls_rendition_selection: {
                        let field_value = match fields_map.get("audio_hls_rendition_selection") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_hls_rendition_selection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_language_selection: {
                        let field_value = match fields_map.get("audio_language_selection") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_language_selection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_pid_selection: {
                        let field_value = match fields_map.get("audio_pid_selection") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_pid_selection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_track_selection: {
                        let field_value = match fields_map.get("audio_track_selection") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_track_selection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
