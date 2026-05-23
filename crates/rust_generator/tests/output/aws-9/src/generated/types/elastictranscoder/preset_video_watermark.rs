#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PresetVideoWatermark {
    /// The horizontal position of the watermark unless you specify a nonzero value for `horzontal_offset`.
    #[builder(into)]
    pub r#horizontal_align: Option<String>,
    /// The amount by which you want the horizontal position of the watermark to be offset from the position specified by `horizontal_align`.
    #[builder(into)]
    pub r#horizontal_offset: Option<String>,
    /// A unique identifier for the settings for one watermark. The value of Id can be up to 40 characters long. You can specify settings for up to four watermarks.
    #[builder(into)]
    pub r#id: Option<String>,
    /// The maximum height of the watermark.
    #[builder(into)]
    pub r#max_height: Option<String>,
    /// The maximum width of the watermark.
    #[builder(into)]
    pub r#max_width: Option<String>,
    /// A percentage that indicates how much you want a watermark to obscure the video in the location where it appears.
    #[builder(into)]
    pub r#opacity: Option<String>,
    /// A value that controls scaling of the watermark. Valid values are: `Fit`, `Stretch`, `ShrinkToFit`
    #[builder(into)]
    pub r#sizing_policy: Option<String>,
    /// A value that determines how Elastic Transcoder interprets values that you specified for `video_watermarks.horizontal_offset`, `video_watermarks.vertical_offset`, `video_watermarks.max_width`, and `video_watermarks.max_height`. Valid values are `Content` and `Frame`.
    #[builder(into)]
    pub r#target: Option<String>,
    /// The vertical position of the watermark unless you specify a nonzero value for `vertical_align`. Valid values are `Top`, `Bottom`, `Center`.
    #[builder(into)]
    pub r#vertical_align: Option<String>,
    /// The amount by which you want the vertical position of the watermark to be offset from the position specified by `vertical_align`
    #[builder(into)]
    pub r#vertical_offset: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PresetVideoWatermark {
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
                    "horizontalAlign",
                    &self.r#horizontal_align,
                ),
                to_pulumi_object_field(
                    "horizontalOffset",
                    &self.r#horizontal_offset,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
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
                    "opacity",
                    &self.r#opacity,
                ),
                to_pulumi_object_field(
                    "sizingPolicy",
                    &self.r#sizing_policy,
                ),
                to_pulumi_object_field(
                    "target",
                    &self.r#target,
                ),
                to_pulumi_object_field(
                    "verticalAlign",
                    &self.r#vertical_align,
                ),
                to_pulumi_object_field(
                    "verticalOffset",
                    &self.r#vertical_offset,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PresetVideoWatermark {
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
                    r#horizontal_align: {
                        let field_value = match fields_map.get("horizontalAlign") {
                            Some(value) => value,
                            None => bail!("Missing field 'horizontalAlign' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#horizontal_offset: {
                        let field_value = match fields_map.get("horizontalOffset") {
                            Some(value) => value,
                            None => bail!("Missing field 'horizontalOffset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#opacity: {
                        let field_value = match fields_map.get("opacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'opacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#target: {
                        let field_value = match fields_map.get("target") {
                            Some(value) => value,
                            None => bail!("Missing field 'target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vertical_align: {
                        let field_value = match fields_map.get("verticalAlign") {
                            Some(value) => value,
                            None => bail!("Missing field 'verticalAlign' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vertical_offset: {
                        let field_value = match fields_map.get("verticalOffset") {
                            Some(value) => value,
                            None => bail!("Missing field 'verticalOffset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
