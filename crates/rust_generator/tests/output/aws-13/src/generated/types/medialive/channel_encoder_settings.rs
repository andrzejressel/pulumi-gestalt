#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettings {
    /// Audio descriptions for the channel. See Audio Descriptions for more details.
    #[builder(into)]
    #[serde(rename = "audioDescriptions")]
    pub r#audio_descriptions: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsAudioDescription>>,
    /// Settings for ad avail blanking. See Avail Blanking for more details.
    #[builder(into)]
    #[serde(rename = "availBlanking")]
    pub r#avail_blanking: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAvailBlanking>>,
    /// Caption Descriptions. See Caption Descriptions for more details.
    #[builder(into)]
    #[serde(rename = "captionDescriptions")]
    pub r#caption_descriptions: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsCaptionDescription>>,
    /// Configuration settings that apply to the event as a whole. See Global Configuration for more details.
    #[builder(into)]
    #[serde(rename = "globalConfiguration")]
    pub r#global_configuration: Option<Box<super::super::types::medialive::ChannelEncoderSettingsGlobalConfiguration>>,
    /// Settings for motion graphics. See Motion Graphics Configuration for more details.
    #[builder(into)]
    #[serde(rename = "motionGraphicsConfiguration")]
    pub r#motion_graphics_configuration: Option<Box<super::super::types::medialive::ChannelEncoderSettingsMotionGraphicsConfiguration>>,
    /// Nielsen configuration settings. See Nielsen Configuration for more details.
    #[builder(into)]
    #[serde(rename = "nielsenConfiguration")]
    pub r#nielsen_configuration: Option<Box<super::super::types::medialive::ChannelEncoderSettingsNielsenConfiguration>>,
    /// Output groups for the channel. See Output Groups for more details.
    #[builder(into)]
    #[serde(rename = "outputGroups")]
    pub r#output_groups: Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroup>,
    /// Contains settings used to acquire and adjust timecode information from inputs. See Timecode Config for more details.
    #[builder(into)]
    #[serde(rename = "timecodeConfig")]
    pub r#timecode_config: Box<super::super::types::medialive::ChannelEncoderSettingsTimecodeConfig>,
    /// Video Descriptions. See Video Descriptions for more details.
    #[builder(into)]
    #[serde(rename = "videoDescriptions")]
    pub r#video_descriptions: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsVideoDescription>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettings {
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
                "audio_descriptions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audio_descriptions,
                )
                .await,
            );
            map.insert(
                "avail_blanking".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#avail_blanking,
                )
                .await,
            );
            map.insert(
                "caption_descriptions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#caption_descriptions,
                )
                .await,
            );
            map.insert(
                "global_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#global_configuration,
                )
                .await,
            );
            map.insert(
                "motion_graphics_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#motion_graphics_configuration,
                )
                .await,
            );
            map.insert(
                "nielsen_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nielsen_configuration,
                )
                .await,
            );
            map.insert(
                "output_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#output_groups,
                )
                .await,
            );
            map.insert(
                "timecode_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timecode_config,
                )
                .await,
            );
            map.insert(
                "video_descriptions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#video_descriptions,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettings {
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
                    r#audio_descriptions: {
                        let field_value = match fields_map.get("audio_descriptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_descriptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#avail_blanking: {
                        let field_value = match fields_map.get("avail_blanking") {
                            Some(value) => value,
                            None => bail!("Missing field 'avail_blanking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#caption_descriptions: {
                        let field_value = match fields_map.get("caption_descriptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'caption_descriptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#global_configuration: {
                        let field_value = match fields_map.get("global_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'global_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#motion_graphics_configuration: {
                        let field_value = match fields_map.get("motion_graphics_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'motion_graphics_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nielsen_configuration: {
                        let field_value = match fields_map.get("nielsen_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'nielsen_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_groups: {
                        let field_value = match fields_map.get("output_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timecode_config: {
                        let field_value = match fields_map.get("timecode_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'timecode_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#video_descriptions: {
                        let field_value = match fields_map.get("video_descriptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'video_descriptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
