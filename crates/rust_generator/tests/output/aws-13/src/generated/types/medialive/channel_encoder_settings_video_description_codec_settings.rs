#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettings {
    /// Frame capture settings. See Frame Capture Settings for more details.
    #[builder(into)]
    #[serde(rename = "frameCaptureSettings")]
    pub r#frame_capture_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsFrameCaptureSettings>>,
    /// H264 settings. See H264 Settings for more details.
    #[builder(into)]
    #[serde(rename = "h264Settings")]
    pub r#h_264_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings>>,
    #[builder(into)]
    #[serde(rename = "h265Settings")]
    pub r#h_265_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsVideoDescriptionCodecSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("frame_capture_settings".to_string(), self.r#frame_capture_settings.to_pulumi_value().await);
            map.insert("h_264_settings".to_string(), self.r#h_264_settings.to_pulumi_value().await);
            map.insert("h_265_settings".to_string(), self.r#h_265_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsVideoDescriptionCodecSettings {
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
                    r#frame_capture_settings: {
                        let field_value = match fields_map.get("frame_capture_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'frame_capture_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsFrameCaptureSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#h_264_settings: {
                        let field_value = match fields_map.get("h_264_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'h_264_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#h_265_settings: {
                        let field_value = match fields_map.get("h_265_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'h_265_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
