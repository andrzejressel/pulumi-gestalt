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

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("audio_descriptions".to_string(), self.r#audio_descriptions.to_pulumi_value().await);
            map.insert("avail_blanking".to_string(), self.r#avail_blanking.to_pulumi_value().await);
            map.insert("caption_descriptions".to_string(), self.r#caption_descriptions.to_pulumi_value().await);
            map.insert("global_configuration".to_string(), self.r#global_configuration.to_pulumi_value().await);
            map.insert("motion_graphics_configuration".to_string(), self.r#motion_graphics_configuration.to_pulumi_value().await);
            map.insert("nielsen_configuration".to_string(), self.r#nielsen_configuration.to_pulumi_value().await);
            map.insert("output_groups".to_string(), self.r#output_groups.to_pulumi_value().await);
            map.insert("timecode_config".to_string(), self.r#timecode_config.to_pulumi_value().await);
            map.insert("video_descriptions".to_string(), self.r#video_descriptions.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettings {
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
                    r#audio_descriptions: {
                        let field_value = match fields_map.get("audio_descriptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_descriptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::medialive::ChannelEncoderSettingsAudioDescription>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#avail_blanking: {
                        let field_value = match fields_map.get("avail_blanking") {
                            Some(value) => value,
                            None => bail!("Missing field 'avail_blanking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsAvailBlanking>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#caption_descriptions: {
                        let field_value = match fields_map.get("caption_descriptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'caption_descriptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::medialive::ChannelEncoderSettingsCaptionDescription>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#global_configuration: {
                        let field_value = match fields_map.get("global_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'global_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsGlobalConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#motion_graphics_configuration: {
                        let field_value = match fields_map.get("motion_graphics_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'motion_graphics_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsMotionGraphicsConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#nielsen_configuration: {
                        let field_value = match fields_map.get("nielsen_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'nielsen_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsNielsenConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_groups: {
                        let field_value = match fields_map.get("output_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroup> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timecode_config: {
                        let field_value = match fields_map.get("timecode_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'timecode_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::medialive::ChannelEncoderSettingsTimecodeConfig> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#video_descriptions: {
                        let field_value = match fields_map.get("video_descriptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'video_descriptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::medialive::ChannelEncoderSettingsVideoDescription>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
