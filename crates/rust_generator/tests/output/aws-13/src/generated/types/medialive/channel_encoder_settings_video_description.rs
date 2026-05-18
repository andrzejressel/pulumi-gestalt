#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsVideoDescription {
    /// The video codec settings. See Video Codec Settings for more details.
    #[builder(into)]
    #[serde(rename = "codecSettings")]
    pub r#codec_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettings>>,
    /// Output video height in pixels.
    #[builder(into)]
    #[serde(rename = "height")]
    pub r#height: Option<i32>,
    /// The name of the video description.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Indicate how to respond to the AFD values that might be in the input video.
    #[builder(into)]
    #[serde(rename = "respondToAfd")]
    pub r#respond_to_afd: Option<String>,
    /// Behavior on how to scale.
    #[builder(into)]
    #[serde(rename = "scalingBehavior")]
    pub r#scaling_behavior: Option<String>,
    /// Changes the strength of the anti-alias filter used for scaling.
    #[builder(into)]
    #[serde(rename = "sharpness")]
    pub r#sharpness: Option<i32>,
    /// Output video width in pixels.
    #[builder(into)]
    #[serde(rename = "width")]
    pub r#width: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsVideoDescription {
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
                    "codec_settings",
                    &self.r#codec_settings,
                ),
                to_pulumi_object_field(
                    "height",
                    &self.r#height,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "respond_to_afd",
                    &self.r#respond_to_afd,
                ),
                to_pulumi_object_field(
                    "scaling_behavior",
                    &self.r#scaling_behavior,
                ),
                to_pulumi_object_field(
                    "sharpness",
                    &self.r#sharpness,
                ),
                to_pulumi_object_field(
                    "width",
                    &self.r#width,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsVideoDescription {
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
                    r#codec_settings: {
                        let field_value = match fields_map.get("codec_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'codec_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#height: {
                        let field_value = match fields_map.get("height") {
                            Some(value) => value,
                            None => bail!("Missing field 'height' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#respond_to_afd: {
                        let field_value = match fields_map.get("respond_to_afd") {
                            Some(value) => value,
                            None => bail!("Missing field 'respond_to_afd' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scaling_behavior: {
                        let field_value = match fields_map.get("scaling_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaling_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sharpness: {
                        let field_value = match fields_map.get("sharpness") {
                            Some(value) => value,
                            None => bail!("Missing field 'sharpness' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#width: {
                        let field_value = match fields_map.get("width") {
                            Some(value) => value,
                            None => bail!("Missing field 'width' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
