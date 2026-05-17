#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PresetVideoWatermark {
    /// The horizontal position of the watermark unless you specify a nonzero value for `horzontal_offset`.
    #[builder(into)]
    #[serde(rename = "horizontalAlign")]
    pub r#horizontal_align: Option<String>,
    /// The amount by which you want the horizontal position of the watermark to be offset from the position specified by `horizontal_align`.
    #[builder(into)]
    #[serde(rename = "horizontalOffset")]
    pub r#horizontal_offset: Option<String>,
    /// A unique identifier for the settings for one watermark. The value of Id can be up to 40 characters long. You can specify settings for up to four watermarks.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The maximum height of the watermark.
    #[builder(into)]
    #[serde(rename = "maxHeight")]
    pub r#max_height: Option<String>,
    /// The maximum width of the watermark.
    #[builder(into)]
    #[serde(rename = "maxWidth")]
    pub r#max_width: Option<String>,
    /// A percentage that indicates how much you want a watermark to obscure the video in the location where it appears.
    #[builder(into)]
    #[serde(rename = "opacity")]
    pub r#opacity: Option<String>,
    /// A value that controls scaling of the watermark. Valid values are: `Fit`, `Stretch`, `ShrinkToFit`
    #[builder(into)]
    #[serde(rename = "sizingPolicy")]
    pub r#sizing_policy: Option<String>,
    /// A value that determines how Elastic Transcoder interprets values that you specified for `video_watermarks.horizontal_offset`, `video_watermarks.vertical_offset`, `video_watermarks.max_width`, and `video_watermarks.max_height`. Valid values are `Content` and `Frame`.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<String>,
    /// The vertical position of the watermark unless you specify a nonzero value for `vertical_align`. Valid values are `Top`, `Bottom`, `Center`.
    #[builder(into)]
    #[serde(rename = "verticalAlign")]
    pub r#vertical_align: Option<String>,
    /// The amount by which you want the vertical position of the watermark to be offset from the position specified by `vertical_align`
    #[builder(into)]
    #[serde(rename = "verticalOffset")]
    pub r#vertical_offset: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PresetVideoWatermark {
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
                "horizontal_align".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#horizontal_align,
                )
                .await,
            );
            map.insert(
                "horizontal_offset".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#horizontal_offset,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
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
                "opacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#opacity,
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
            map.insert(
                "target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target,
                )
                .await,
            );
            map.insert(
                "vertical_align".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vertical_align,
                )
                .await,
            );
            map.insert(
                "vertical_offset".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vertical_offset,
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
                        let field_value = match fields_map.get("horizontal_align") {
                            Some(value) => value,
                            None => bail!("Missing field 'horizontal_align' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#horizontal_offset: {
                        let field_value = match fields_map.get("horizontal_offset") {
                            Some(value) => value,
                            None => bail!("Missing field 'horizontal_offset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#opacity: {
                        let field_value = match fields_map.get("opacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'opacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#target: {
                        let field_value = match fields_map.get("target") {
                            Some(value) => value,
                            None => bail!("Missing field 'target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vertical_align: {
                        let field_value = match fields_map.get("vertical_align") {
                            Some(value) => value,
                            None => bail!("Missing field 'vertical_align' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vertical_offset: {
                        let field_value = match fields_map.get("vertical_offset") {
                            Some(value) => value,
                            None => bail!("Missing field 'vertical_offset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
