#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettings {
    /// Sets the colorspace metadata to be passed through.
    #[builder(into)]
    #[serde(rename = "colorSpacePassthroughSettings")]
    pub r#color_space_passthrough_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsColorSpacePassthroughSettings>>,
    /// Set the colorspace to Dolby Vision81.
    #[builder(into)]
    #[serde(rename = "dolbyVision81Settings")]
    pub r#dolby_vision_81_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsDolbyVision81Settings>>,
    /// Set the colorspace to be HDR10. See H265 HDR10 Settings for more details.
    #[builder(into)]
    #[serde(rename = "hdr10Settings")]
    pub r#hdr_10_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsHdr10Settings>>,
    /// Set the colorspace to Rec. 601.
    #[builder(into)]
    #[serde(rename = "rec601Settings")]
    pub r#rec_601_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec601Settings>>,
    /// Set the colorspace to Rec. 709.
    #[builder(into)]
    #[serde(rename = "rec709Settings")]
    pub r#rec_709_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec709Settings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "color_space_passthrough_settings",
                    &self.r#color_space_passthrough_settings,
                ),
                to_pulumi_object_field(
                    "dolby_vision_81_settings",
                    &self.r#dolby_vision_81_settings,
                ),
                to_pulumi_object_field(
                    "hdr_10_settings",
                    &self.r#hdr_10_settings,
                ),
                to_pulumi_object_field(
                    "rec_601_settings",
                    &self.r#rec_601_settings,
                ),
                to_pulumi_object_field(
                    "rec_709_settings",
                    &self.r#rec_709_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettings {
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
                    r#color_space_passthrough_settings: {
                        let field_value = match fields_map.get("color_space_passthrough_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'color_space_passthrough_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dolby_vision_81_settings: {
                        let field_value = match fields_map.get("dolby_vision_81_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dolby_vision_81_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hdr_10_settings: {
                        let field_value = match fields_map.get("hdr_10_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'hdr_10_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rec_601_settings: {
                        let field_value = match fields_map.get("rec_601_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'rec_601_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rec_709_settings: {
                        let field_value = match fields_map.get("rec_709_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'rec_709_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
