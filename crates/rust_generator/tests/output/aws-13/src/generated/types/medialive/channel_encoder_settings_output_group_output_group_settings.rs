#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettings {
    /// Archive group settings. See Archive Group Settings for more details.
    #[builder(into)]
    #[serde(rename = "archiveGroupSettings")]
    pub r#archive_group_settings: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSetting>>,
    #[builder(into)]
    #[serde(rename = "frameCaptureGroupSettings")]
    pub r#frame_capture_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettings>>,
    #[builder(into)]
    #[serde(rename = "hlsGroupSettings")]
    pub r#hls_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings>>,
    /// Media package group settings. See Media Package Group Settings for more details.
    #[builder(into)]
    #[serde(rename = "mediaPackageGroupSettings")]
    pub r#media_package_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettings>>,
    #[builder(into)]
    #[serde(rename = "msSmoothGroupSettings")]
    pub r#ms_smooth_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings>>,
    #[builder(into)]
    #[serde(rename = "multiplexGroupSettings")]
    pub r#multiplex_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMultiplexGroupSettings>>,
    /// RTMP group settings. See RTMP Group Settings for more details.
    #[builder(into)]
    #[serde(rename = "rtmpGroupSettings")]
    pub r#rtmp_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings>>,
    #[builder(into)]
    #[serde(rename = "udpGroupSettings")]
    pub r#udp_group_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("archive_group_settings".to_string(), self.r#archive_group_settings.to_pulumi_value().await);
            map.insert("frame_capture_group_settings".to_string(), self.r#frame_capture_group_settings.to_pulumi_value().await);
            map.insert("hls_group_settings".to_string(), self.r#hls_group_settings.to_pulumi_value().await);
            map.insert("media_package_group_settings".to_string(), self.r#media_package_group_settings.to_pulumi_value().await);
            map.insert("ms_smooth_group_settings".to_string(), self.r#ms_smooth_group_settings.to_pulumi_value().await);
            map.insert("multiplex_group_settings".to_string(), self.r#multiplex_group_settings.to_pulumi_value().await);
            map.insert("rtmp_group_settings".to_string(), self.r#rtmp_group_settings.to_pulumi_value().await);
            map.insert("udp_group_settings".to_string(), self.r#udp_group_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettings {
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
                    r#archive_group_settings: {
                        let field_value = match fields_map.get("archive_group_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'archive_group_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSetting>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#frame_capture_group_settings: {
                        let field_value = match fields_map.get("frame_capture_group_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'frame_capture_group_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hls_group_settings: {
                        let field_value = match fields_map.get("hls_group_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'hls_group_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#media_package_group_settings: {
                        let field_value = match fields_map.get("media_package_group_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'media_package_group_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ms_smooth_group_settings: {
                        let field_value = match fields_map.get("ms_smooth_group_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'ms_smooth_group_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#multiplex_group_settings: {
                        let field_value = match fields_map.get("multiplex_group_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiplex_group_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMultiplexGroupSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rtmp_group_settings: {
                        let field_value = match fields_map.get("rtmp_group_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'rtmp_group_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#udp_group_settings: {
                        let field_value = match fields_map.get("udp_group_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'udp_group_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
