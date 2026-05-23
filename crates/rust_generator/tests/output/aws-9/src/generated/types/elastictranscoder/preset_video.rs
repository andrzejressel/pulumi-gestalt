#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PresetVideo {
    /// The display aspect ratio of the video in the output file. Valid values are: `auto`, `1:1`, `4:3`, `3:2`, `16:9`. (Note; to better control resolution and aspect ratio of output videos, we recommend that you use the values `max_width`, `max_height`, `sizing_policy`, `padding_policy`, and `display_aspect_ratio` instead of `resolution` and `aspect_ratio`.)
    #[builder(into)]
    pub r#aspect_ratio: Option<String>,
    /// The bit rate of the video stream in the output file, in kilobits/second. You can configure variable bit rate or constant bit rate encoding.
    #[builder(into)]
    pub r#bit_rate: Option<String>,
    /// The video codec for the output file. Valid values are `gif`, `H.264`, `mpeg2`, `vp8`, and `vp9`.
    #[builder(into)]
    pub r#codec: Option<String>,
    /// The value that Elastic Transcoder adds to the metadata in the output file. If you set DisplayAspectRatio to auto, Elastic Transcoder chooses an aspect ratio that ensures square pixels. If you specify another option, Elastic Transcoder sets that value in the output file.
    #[builder(into)]
    pub r#display_aspect_ratio: Option<String>,
    /// Whether to use a fixed value for Video:FixedGOP. Not applicable for containers of type gif. Valid values are true and false. Also known as, Fixed Number of Frames Between Keyframes.
    #[builder(into)]
    pub r#fixed_gop: Option<String>,
    /// The frames per second for the video stream in the output file. The following values are valid: `auto`, `10`, `15`, `23.97`, `24`, `25`, `29.97`, `30`, `50`, `60`.
    #[builder(into)]
    pub r#frame_rate: Option<String>,
    /// The maximum number of frames between key frames. Not applicable for containers of type gif.
    #[builder(into)]
    pub r#keyframes_max_dist: Option<String>,
    /// If you specify auto for FrameRate, Elastic Transcoder uses the frame rate of the input video for the frame rate of the output video, up to the maximum frame rate. If you do not specify a MaxFrameRate, Elastic Transcoder will use a default of 30.
    #[builder(into)]
    pub r#max_frame_rate: Option<String>,
    /// The maximum height of the output video in pixels. If you specify auto, Elastic Transcoder uses 1080 (Full HD) as the default value. If you specify a numeric value, enter an even integer between 96 and 3072, inclusive.
    #[builder(into)]
    pub r#max_height: Option<String>,
    /// The maximum width of the output video in pixels. If you specify auto, Elastic Transcoder uses 1920 (Full HD) as the default value. If you specify a numeric value, enter an even integer between 128 and 4096, inclusive.
    #[builder(into)]
    pub r#max_width: Option<String>,
    /// When you set PaddingPolicy to Pad, Elastic Transcoder might add black bars to the top and bottom and/or left and right sides of the output video to make the total size of the output video match the values that you specified for `max_width` and `max_height`.
    #[builder(into)]
    pub r#padding_policy: Option<String>,
    /// The width and height of the video in the output file, in pixels. Valid values are `auto` and `widthxheight`. (see note for `aspect_ratio`)
    #[builder(into)]
    pub r#resolution: Option<String>,
    /// A value that controls scaling of the output video. Valid values are: `Fit`, `Fill`, `Stretch`, `Keep`, `ShrinkToFit`, `ShrinkToFill`.
    #[builder(into)]
    pub r#sizing_policy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PresetVideo {
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
                    "aspectRatio",
                    &self.r#aspect_ratio,
                ),
                to_pulumi_object_field(
                    "bitRate",
                    &self.r#bit_rate,
                ),
                to_pulumi_object_field(
                    "codec",
                    &self.r#codec,
                ),
                to_pulumi_object_field(
                    "displayAspectRatio",
                    &self.r#display_aspect_ratio,
                ),
                to_pulumi_object_field(
                    "fixedGop",
                    &self.r#fixed_gop,
                ),
                to_pulumi_object_field(
                    "frameRate",
                    &self.r#frame_rate,
                ),
                to_pulumi_object_field(
                    "keyframesMaxDist",
                    &self.r#keyframes_max_dist,
                ),
                to_pulumi_object_field(
                    "maxFrameRate",
                    &self.r#max_frame_rate,
                ),
                to_pulumi_object_field(
                    "maxHeight",
                    &self.r#max_height,
                ),
                to_pulumi_object_field(
                    "maxWidth",
                    &self.r#max_width,
                ),
                to_pulumi_object_field(
                    "paddingPolicy",
                    &self.r#padding_policy,
                ),
                to_pulumi_object_field(
                    "resolution",
                    &self.r#resolution,
                ),
                to_pulumi_object_field(
                    "sizingPolicy",
                    &self.r#sizing_policy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("aspectRatio") {
                            Some(value) => value,
                            None => bail!("Missing field 'aspectRatio' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bit_rate: {
                        let field_value = match fields_map.get("bitRate") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitRate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("displayAspectRatio") {
                            Some(value) => value,
                            None => bail!("Missing field 'displayAspectRatio' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fixed_gop: {
                        let field_value = match fields_map.get("fixedGop") {
                            Some(value) => value,
                            None => bail!("Missing field 'fixedGop' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frame_rate: {
                        let field_value = match fields_map.get("frameRate") {
                            Some(value) => value,
                            None => bail!("Missing field 'frameRate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#keyframes_max_dist: {
                        let field_value = match fields_map.get("keyframesMaxDist") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyframesMaxDist' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_frame_rate: {
                        let field_value = match fields_map.get("maxFrameRate") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxFrameRate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_height: {
                        let field_value = match fields_map.get("maxHeight") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxHeight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_width: {
                        let field_value = match fields_map.get("maxWidth") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxWidth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#padding_policy: {
                        let field_value = match fields_map.get("paddingPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'paddingPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("sizingPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'sizingPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
