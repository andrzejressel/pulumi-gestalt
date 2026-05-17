#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PresetVideo {
    /// The display aspect ratio of the video in the output file. Valid values are: `auto`, `1:1`, `4:3`, `3:2`, `16:9`. (Note; to better control resolution and aspect ratio of output videos, we recommend that you use the values `max_width`, `max_height`, `sizing_policy`, `padding_policy`, and `display_aspect_ratio` instead of `resolution` and `aspect_ratio`.)
    #[builder(into)]
    #[serde(rename = "aspectRatio")]
    pub r#aspect_ratio: Option<String>,
    /// The bit rate of the video stream in the output file, in kilobits/second. You can configure variable bit rate or constant bit rate encoding.
    #[builder(into)]
    #[serde(rename = "bitRate")]
    pub r#bit_rate: Option<String>,
    /// The video codec for the output file. Valid values are `gif`, `H.264`, `mpeg2`, `vp8`, and `vp9`.
    #[builder(into)]
    #[serde(rename = "codec")]
    pub r#codec: Option<String>,
    /// The value that Elastic Transcoder adds to the metadata in the output file. If you set DisplayAspectRatio to auto, Elastic Transcoder chooses an aspect ratio that ensures square pixels. If you specify another option, Elastic Transcoder sets that value in the output file.
    #[builder(into)]
    #[serde(rename = "displayAspectRatio")]
    pub r#display_aspect_ratio: Option<String>,
    /// Whether to use a fixed value for Video:FixedGOP. Not applicable for containers of type gif. Valid values are true and false. Also known as, Fixed Number of Frames Between Keyframes.
    #[builder(into)]
    #[serde(rename = "fixedGop")]
    pub r#fixed_gop: Option<String>,
    /// The frames per second for the video stream in the output file. The following values are valid: `auto`, `10`, `15`, `23.97`, `24`, `25`, `29.97`, `30`, `50`, `60`.
    #[builder(into)]
    #[serde(rename = "frameRate")]
    pub r#frame_rate: Option<String>,
    /// The maximum number of frames between key frames. Not applicable for containers of type gif.
    #[builder(into)]
    #[serde(rename = "keyframesMaxDist")]
    pub r#keyframes_max_dist: Option<String>,
    /// If you specify auto for FrameRate, Elastic Transcoder uses the frame rate of the input video for the frame rate of the output video, up to the maximum frame rate. If you do not specify a MaxFrameRate, Elastic Transcoder will use a default of 30.
    #[builder(into)]
    #[serde(rename = "maxFrameRate")]
    pub r#max_frame_rate: Option<String>,
    /// The maximum height of the output video in pixels. If you specify auto, Elastic Transcoder uses 1080 (Full HD) as the default value. If you specify a numeric value, enter an even integer between 96 and 3072, inclusive.
    #[builder(into)]
    #[serde(rename = "maxHeight")]
    pub r#max_height: Option<String>,
    /// The maximum width of the output video in pixels. If you specify auto, Elastic Transcoder uses 1920 (Full HD) as the default value. If you specify a numeric value, enter an even integer between 128 and 4096, inclusive.
    #[builder(into)]
    #[serde(rename = "maxWidth")]
    pub r#max_width: Option<String>,
    /// When you set PaddingPolicy to Pad, Elastic Transcoder might add black bars to the top and bottom and/or left and right sides of the output video to make the total size of the output video match the values that you specified for `max_width` and `max_height`.
    #[builder(into)]
    #[serde(rename = "paddingPolicy")]
    pub r#padding_policy: Option<String>,
    /// The width and height of the video in the output file, in pixels. Valid values are `auto` and `widthxheight`. (see note for `aspect_ratio`)
    #[builder(into)]
    #[serde(rename = "resolution")]
    pub r#resolution: Option<String>,
    /// A value that controls scaling of the output video. Valid values are: `Fit`, `Fill`, `Stretch`, `Keep`, `ShrinkToFit`, `ShrinkToFill`.
    #[builder(into)]
    #[serde(rename = "sizingPolicy")]
    pub r#sizing_policy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PresetVideo {
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
                "aspect_ratio".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aspect_ratio,
                )
                .await,
            );
            map.insert(
                "bit_rate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bit_rate,
                )
                .await,
            );
            map.insert(
                "codec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#codec,
                )
                .await,
            );
            map.insert(
                "display_aspect_ratio".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#display_aspect_ratio,
                )
                .await,
            );
            map.insert(
                "fixed_gop".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fixed_gop,
                )
                .await,
            );
            map.insert(
                "frame_rate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#frame_rate,
                )
                .await,
            );
            map.insert(
                "keyframes_max_dist".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#keyframes_max_dist,
                )
                .await,
            );
            map.insert(
                "max_frame_rate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_frame_rate,
                )
                .await,
            );
            map.insert(
                "max_height".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_height,
                )
                .await,
            );
            map.insert(
                "max_width".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_width,
                )
                .await,
            );
            map.insert(
                "padding_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#padding_policy,
                )
                .await,
            );
            map.insert(
                "resolution".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resolution,
                )
                .await,
            );
            map.insert(
                "sizing_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sizing_policy,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PresetVideo {
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
                    r#aspect_ratio: {
                        let field_value = match fields_map.get("aspect_ratio") {
                            Some(value) => value,
                            None => bail!("Missing field 'aspect_ratio' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bit_rate: {
                        let field_value = match fields_map.get("bit_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'bit_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codec: {
                        let field_value = match fields_map.get("codec") {
                            Some(value) => value,
                            None => bail!("Missing field 'codec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_aspect_ratio: {
                        let field_value = match fields_map.get("display_aspect_ratio") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_aspect_ratio' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fixed_gop: {
                        let field_value = match fields_map.get("fixed_gop") {
                            Some(value) => value,
                            None => bail!("Missing field 'fixed_gop' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frame_rate: {
                        let field_value = match fields_map.get("frame_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'frame_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#keyframes_max_dist: {
                        let field_value = match fields_map.get("keyframes_max_dist") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyframes_max_dist' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_frame_rate: {
                        let field_value = match fields_map.get("max_frame_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_frame_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_height: {
                        let field_value = match fields_map.get("max_height") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_height' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_width: {
                        let field_value = match fields_map.get("max_width") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_width' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#padding_policy: {
                        let field_value = match fields_map.get("padding_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'padding_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resolution: {
                        let field_value = match fields_map.get("resolution") {
                            Some(value) => value,
                            None => bail!("Missing field 'resolution' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sizing_policy: {
                        let field_value = match fields_map.get("sizing_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'sizing_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
