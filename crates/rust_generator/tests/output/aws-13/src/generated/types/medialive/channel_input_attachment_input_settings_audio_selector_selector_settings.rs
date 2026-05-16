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
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("audio_hls_rendition_selection".to_string(), self.r#audio_hls_rendition_selection.to_pulumi_value().await);
            map.insert("audio_language_selection".to_string(), self.r#audio_language_selection.to_pulumi_value().await);
            map.insert("audio_pid_selection".to_string(), self.r#audio_pid_selection.to_pulumi_value().await);
            map.insert("audio_track_selection".to_string(), self.r#audio_track_selection.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettings {
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
                    r#audio_hls_rendition_selection: {
                        let field_value = match fields_map.get("audio_hls_rendition_selection") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_hls_rendition_selection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioHlsRenditionSelection>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#audio_language_selection: {
                        let field_value = match fields_map.get("audio_language_selection") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_language_selection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioLanguageSelection>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#audio_pid_selection: {
                        let field_value = match fields_map.get("audio_pid_selection") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_pid_selection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioPidSelection>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#audio_track_selection: {
                        let field_value = match fields_map.get("audio_track_selection") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_track_selection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelection>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
