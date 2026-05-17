#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettings {
    /// If no explicit xPosition or yPosition is provided, setting alignment to centered will place the captions at the bottom center of the output. Similarly, setting a left alignment will align captions to the bottom left of the output. If x and y positions are given in conjunction with the alignment parameter, the font will be justified (either left or centered) relative to those coordinates. Selecting “smart” justification will left-justify live subtitles and center-justify pre-recorded subtitles. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "alignment")]
    pub r#alignment: Option<String>,
    /// Specifies the color of the rectangle behind the captions. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Option<String>,
    /// Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter out is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "backgroundOpacity")]
    pub r#background_opacity: Option<i32>,
    /// External font file used for caption burn-in. File extension must be ‘ttf’ or ‘tte’. Although the user can select output fonts for many different types of input captions, embedded, STL and teletext sources use a strict grid system. Using external fonts with these caption sources could cause unexpected display of proportional fonts. All burn-in and DVB-Sub font settings must match. See Font for more details.
    #[builder(into)]
    #[serde(rename = "font")]
    pub r#font: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettingsFont>>,
    /// Specifies the color of the burned-in captions. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "fontColor")]
    pub r#font_color: Option<String>,
    /// Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "fontOpacity")]
    pub r#font_opacity: Option<i32>,
    /// Font resolution in DPI (dots per inch); default is 96 dpi. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "fontResolution")]
    pub r#font_resolution: Option<i32>,
    /// When set to ‘auto’ fontSize will scale depending on the size of the output. Giving a positive integer will specify the exact font size in points. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "fontSize")]
    pub r#font_size: Option<String>,
    /// Specifies font outline color. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "outlineColor")]
    pub r#outline_color: String,
    /// Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "outlineSize")]
    pub r#outline_size: Option<i32>,
    /// Specifies the color of the shadow cast by the captions. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "shadowColor")]
    pub r#shadow_color: Option<String>,
    /// Specifies the opacity of the shadow. 255 is opaque; 0 is transparent. Leaving this parameter out is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "shadowOpacity")]
    pub r#shadow_opacity: Option<i32>,
    /// Specifies the horizontal offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels to the left. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "shadowXOffset")]
    pub r#shadow_x_offset: Option<i32>,
    /// Specifies the vertical offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels above the text. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "shadowYOffset")]
    pub r#shadow_y_offset: Option<i32>,
    /// Controls whether a fixed grid size will be used to generate the output subtitles bitmap. Only applicable for Teletext inputs and DVB-Sub/Burn-in outputs.
    #[builder(into)]
    #[serde(rename = "teletextGridControl")]
    pub r#teletext_grid_control: String,
    /// Specifies the horizontal position of the caption relative to the left side of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the left of the output. If no explicit xPosition is provided, the horizontal caption position will be determined by the alignment parameter. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "xPosition")]
    pub r#x_position: Option<i32>,
    /// Specifies the vertical position of the caption relative to the top of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the top of the output. If no explicit yPosition is provided, the caption will be positioned towards the bottom of the output. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "yPosition")]
    pub r#y_position: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "alignment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#alignment,
                )
                .await,
            );
            map.insert(
                "background_color".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#background_color,
                )
                .await,
            );
            map.insert(
                "background_opacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#background_opacity,
                )
                .await,
            );
            map.insert(
                "font".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#font,
                )
                .await,
            );
            map.insert(
                "font_color".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#font_color,
                )
                .await,
            );
            map.insert(
                "font_opacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#font_opacity,
                )
                .await,
            );
            map.insert(
                "font_resolution".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#font_resolution,
                )
                .await,
            );
            map.insert(
                "font_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#font_size,
                )
                .await,
            );
            map.insert(
                "outline_color".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#outline_color,
                )
                .await,
            );
            map.insert(
                "outline_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#outline_size,
                )
                .await,
            );
            map.insert(
                "shadow_color".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#shadow_color,
                )
                .await,
            );
            map.insert(
                "shadow_opacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#shadow_opacity,
                )
                .await,
            );
            map.insert(
                "shadow_x_offset".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#shadow_x_offset,
                )
                .await,
            );
            map.insert(
                "shadow_y_offset".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#shadow_y_offset,
                )
                .await,
            );
            map.insert(
                "teletext_grid_control".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#teletext_grid_control,
                )
                .await,
            );
            map.insert(
                "x_position".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#x_position,
                )
                .await,
            );
            map.insert(
                "y_position".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#y_position,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettings {
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
                    r#alignment: {
                        let field_value = match fields_map.get("alignment") {
                            Some(value) => value,
                            None => bail!("Missing field 'alignment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#background_color: {
                        let field_value = match fields_map.get("background_color") {
                            Some(value) => value,
                            None => bail!("Missing field 'background_color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#background_opacity: {
                        let field_value = match fields_map.get("background_opacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'background_opacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#font: {
                        let field_value = match fields_map.get("font") {
                            Some(value) => value,
                            None => bail!("Missing field 'font' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#font_color: {
                        let field_value = match fields_map.get("font_color") {
                            Some(value) => value,
                            None => bail!("Missing field 'font_color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#font_opacity: {
                        let field_value = match fields_map.get("font_opacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'font_opacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#font_resolution: {
                        let field_value = match fields_map.get("font_resolution") {
                            Some(value) => value,
                            None => bail!("Missing field 'font_resolution' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#font_size: {
                        let field_value = match fields_map.get("font_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'font_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outline_color: {
                        let field_value = match fields_map.get("outline_color") {
                            Some(value) => value,
                            None => bail!("Missing field 'outline_color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outline_size: {
                        let field_value = match fields_map.get("outline_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'outline_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shadow_color: {
                        let field_value = match fields_map.get("shadow_color") {
                            Some(value) => value,
                            None => bail!("Missing field 'shadow_color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shadow_opacity: {
                        let field_value = match fields_map.get("shadow_opacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'shadow_opacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shadow_x_offset: {
                        let field_value = match fields_map.get("shadow_x_offset") {
                            Some(value) => value,
                            None => bail!("Missing field 'shadow_x_offset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shadow_y_offset: {
                        let field_value = match fields_map.get("shadow_y_offset") {
                            Some(value) => value,
                            None => bail!("Missing field 'shadow_y_offset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#teletext_grid_control: {
                        let field_value = match fields_map.get("teletext_grid_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'teletext_grid_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#x_position: {
                        let field_value = match fields_map.get("x_position") {
                            Some(value) => value,
                            None => bail!("Missing field 'x_position' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#y_position: {
                        let field_value = match fields_map.get("y_position") {
                            Some(value) => value,
                            None => bail!("Missing field 'y_position' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
