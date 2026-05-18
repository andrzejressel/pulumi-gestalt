#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettings {
    /// Complete this field if you want to include the name of the copyright holder in the copyright tag in the captions metadata.
    #[builder(into)]
    #[serde(rename = "copyrightHolder")]
    pub r#copyright_holder: Option<String>,
    /// Specifies how to handle the gap between the lines (in multi-line captions). - enabled: Fill with the captions background color (as specified in the input captions). - disabled: Leave the gap unfilled.
    #[builder(into)]
    #[serde(rename = "fillLineGap")]
    pub r#fill_line_gap: Option<String>,
    /// Specifies the font family to include in the font data attached to the EBU-TT captions. Valid only if styleControl is set to include. If you leave this field empty, the font family is set to “monospaced”. (If styleControl is set to exclude, the font family is always set to “monospaced”.) You specify only the font family. All other style information (color, bold, position and so on) is copied from the input captions. The size is always set to 100% to allow the downstream player to choose the size. - Enter a list of font families, as a comma-separated list of font names, in order of preference. The name can be a font family (such as “Arial”), or a generic font family (such as “serif”), or “default” (to let the downstream player choose the font). - Leave blank to set the family to “monospace”.
    #[builder(into)]
    #[serde(rename = "fontFamily")]
    pub r#font_family: Option<String>,
    /// Specifies the style information (font color, font position, and so on) to include in the font data that is attached to the EBU-TT captions. - include: Take the style information (font color, font position, and so on) from the source captions and include that information in the font data attached to the EBU-TT captions. This option is valid only if the source captions are Embedded or Teletext. - exclude: In the font data attached to the EBU-TT captions, set the font family to “monospaced”. Do not include any other style information.
    #[builder(into)]
    #[serde(rename = "styleControl")]
    pub r#style_control: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettings {
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
                    "copyright_holder",
                    &self.r#copyright_holder,
                ),
                to_pulumi_object_field(
                    "fill_line_gap",
                    &self.r#fill_line_gap,
                ),
                to_pulumi_object_field(
                    "font_family",
                    &self.r#font_family,
                ),
                to_pulumi_object_field(
                    "style_control",
                    &self.r#style_control,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettings {
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
                    r#copyright_holder: {
                        let field_value = match fields_map.get("copyright_holder") {
                            Some(value) => value,
                            None => bail!("Missing field 'copyright_holder' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fill_line_gap: {
                        let field_value = match fields_map.get("fill_line_gap") {
                            Some(value) => value,
                            None => bail!("Missing field 'fill_line_gap' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#font_family: {
                        let field_value = match fields_map.get("font_family") {
                            Some(value) => value,
                            None => bail!("Missing field 'font_family' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#style_control: {
                        let field_value = match fields_map.get("style_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'style_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
