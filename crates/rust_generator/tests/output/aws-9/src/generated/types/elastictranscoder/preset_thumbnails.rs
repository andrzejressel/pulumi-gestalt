#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PresetThumbnails {
    /// The aspect ratio of thumbnails. The following values are valid: auto, 1:1, 4:3, 3:2, 16:9
    #[builder(into)]
    #[serde(rename = "aspectRatio")]
    pub r#aspect_ratio: Option<String>,
    /// The format of thumbnails, if any. Valid formats are jpg and png.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Option<String>,
    /// The approximate number of seconds between thumbnails. The value must be an integer. The actual interval can vary by several seconds from one thumbnail to the next.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Option<String>,
    /// The maximum height of thumbnails, in pixels. If you specify auto, Elastic Transcoder uses 1080 (Full HD) as the default value. If you specify a numeric value, enter an even integer between 32 and 3072, inclusive.
    #[builder(into)]
    #[serde(rename = "maxHeight")]
    pub r#max_height: Option<String>,
    /// The maximum width of thumbnails, in pixels. If you specify auto, Elastic Transcoder uses 1920 (Full HD) as the default value. If you specify a numeric value, enter an even integer between 32 and 4096, inclusive.
    #[builder(into)]
    #[serde(rename = "maxWidth")]
    pub r#max_width: Option<String>,
    /// When you set PaddingPolicy to Pad, Elastic Transcoder might add black bars to the top and bottom and/or left and right sides of thumbnails to make the total size of the thumbnails match the values that you specified for thumbnail MaxWidth and MaxHeight settings.
    #[builder(into)]
    #[serde(rename = "paddingPolicy")]
    pub r#padding_policy: Option<String>,
    /// The width and height of thumbnail files in pixels, in the format WidthxHeight, where both values are even integers. The values cannot exceed the width and height that you specified in the Video:Resolution object. (To better control resolution and aspect ratio of thumbnails, we recommend that you use the thumbnail values `max_width`, `max_height`, `sizing_policy`, and `padding_policy` instead of `resolution` and `aspect_ratio`. The two groups of settings are mutually exclusive. Do not use them together)
    #[builder(into)]
    #[serde(rename = "resolution")]
    pub r#resolution: Option<String>,
    /// A value that controls scaling of thumbnails. Valid values are: `Fit`, `Fill`, `Stretch`, `Keep`, `ShrinkToFit`, and `ShrinkToFill`.
    #[builder(into)]
    #[serde(rename = "sizingPolicy")]
    pub r#sizing_policy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PresetThumbnails {
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
                "format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#format,
                )
                .await,
            );
            map.insert(
                "interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#interval,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PresetThumbnails {
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
                    r#format: {
                        let field_value = match fields_map.get("format") {
                            Some(value) => value,
                            None => bail!("Missing field 'format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interval: {
                        let field_value = match fields_map.get("interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
