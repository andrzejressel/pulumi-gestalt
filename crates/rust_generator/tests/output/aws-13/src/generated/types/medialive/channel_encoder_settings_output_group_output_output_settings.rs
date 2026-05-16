#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettings {
    /// Archive output settings. See Archive Output Settings for more details.
    #[builder(into)]
    #[serde(rename = "archiveOutputSettings")]
    pub r#archive_output_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings>>,
    #[builder(into)]
    #[serde(rename = "frameCaptureOutputSettings")]
    pub r#frame_capture_output_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsFrameCaptureOutputSettings>>,
    #[builder(into)]
    #[serde(rename = "hlsOutputSettings")]
    pub r#hls_output_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings>>,
    /// Media package output settings. This can be set as an empty block.
    #[builder(into)]
    #[serde(rename = "mediaPackageOutputSettings")]
    pub r#media_package_output_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsMediaPackageOutputSettings>>,
    #[builder(into)]
    #[serde(rename = "msSmoothOutputSettings")]
    pub r#ms_smooth_output_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsMsSmoothOutputSettings>>,
    /// Multiplex output settings. See Multiplex Output Settings for more details.
    #[builder(into)]
    #[serde(rename = "multiplexOutputSettings")]
    pub r#multiplex_output_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettings>>,
    /// RTMP output settings. See RTMP Output Settings for more details.
    #[builder(into)]
    #[serde(rename = "rtmpOutputSettings")]
    pub r#rtmp_output_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettings>>,
    /// UDP output settings. See UDP Output Settings for more details.
    #[builder(into)]
    #[serde(rename = "udpOutputSettings")]
    pub r#udp_output_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("archive_output_settings".to_string(), self.r#archive_output_settings.to_pulumi_value().await);
            map.insert("frame_capture_output_settings".to_string(), self.r#frame_capture_output_settings.to_pulumi_value().await);
            map.insert("hls_output_settings".to_string(), self.r#hls_output_settings.to_pulumi_value().await);
            map.insert("media_package_output_settings".to_string(), self.r#media_package_output_settings.to_pulumi_value().await);
            map.insert("ms_smooth_output_settings".to_string(), self.r#ms_smooth_output_settings.to_pulumi_value().await);
            map.insert("multiplex_output_settings".to_string(), self.r#multiplex_output_settings.to_pulumi_value().await);
            map.insert("rtmp_output_settings".to_string(), self.r#rtmp_output_settings.to_pulumi_value().await);
            map.insert("udp_output_settings".to_string(), self.r#udp_output_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettings {
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
                    r#archive_output_settings: {
                        let field_value = match fields_map.get("archive_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'archive_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#frame_capture_output_settings: {
                        let field_value = match fields_map.get("frame_capture_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'frame_capture_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsFrameCaptureOutputSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hls_output_settings: {
                        let field_value = match fields_map.get("hls_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'hls_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#media_package_output_settings: {
                        let field_value = match fields_map.get("media_package_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'media_package_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsMediaPackageOutputSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ms_smooth_output_settings: {
                        let field_value = match fields_map.get("ms_smooth_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'ms_smooth_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsMsSmoothOutputSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#multiplex_output_settings: {
                        let field_value = match fields_map.get("multiplex_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiplex_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rtmp_output_settings: {
                        let field_value = match fields_map.get("rtmp_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'rtmp_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#udp_output_settings: {
                        let field_value = match fields_map.get("udp_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'udp_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
