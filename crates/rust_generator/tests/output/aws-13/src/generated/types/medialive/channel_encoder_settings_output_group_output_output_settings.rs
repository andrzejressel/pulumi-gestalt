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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "archive_output_settings",
                    &self.r#archive_output_settings,
                ),
                to_pulumi_object_field(
                    "frame_capture_output_settings",
                    &self.r#frame_capture_output_settings,
                ),
                to_pulumi_object_field(
                    "hls_output_settings",
                    &self.r#hls_output_settings,
                ),
                to_pulumi_object_field(
                    "media_package_output_settings",
                    &self.r#media_package_output_settings,
                ),
                to_pulumi_object_field(
                    "ms_smooth_output_settings",
                    &self.r#ms_smooth_output_settings,
                ),
                to_pulumi_object_field(
                    "multiplex_output_settings",
                    &self.r#multiplex_output_settings,
                ),
                to_pulumi_object_field(
                    "rtmp_output_settings",
                    &self.r#rtmp_output_settings,
                ),
                to_pulumi_object_field(
                    "udp_output_settings",
                    &self.r#udp_output_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettings {
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
                    r#archive_output_settings: {
                        let field_value = match fields_map.get("archive_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'archive_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frame_capture_output_settings: {
                        let field_value = match fields_map.get("frame_capture_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'frame_capture_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hls_output_settings: {
                        let field_value = match fields_map.get("hls_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'hls_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#media_package_output_settings: {
                        let field_value = match fields_map.get("media_package_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'media_package_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ms_smooth_output_settings: {
                        let field_value = match fields_map.get("ms_smooth_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'ms_smooth_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multiplex_output_settings: {
                        let field_value = match fields_map.get("multiplex_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiplex_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rtmp_output_settings: {
                        let field_value = match fields_map.get("rtmp_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'rtmp_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#udp_output_settings: {
                        let field_value = match fields_map.get("udp_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'udp_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
