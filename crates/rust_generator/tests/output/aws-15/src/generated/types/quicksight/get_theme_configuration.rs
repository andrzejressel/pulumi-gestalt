#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetThemeConfiguration {
    /// Color properties that apply to chart data colors. See data_color_palette.
    #[builder(into)]
    #[serde(rename = "dataColorPalettes")]
    pub r#data_color_palettes: Vec<super::super::types::quicksight::GetThemeConfigurationDataColorPalette>,
    /// Display options related to sheets. See sheet.
    #[builder(into)]
    #[serde(rename = "sheets")]
    pub r#sheets: Vec<super::super::types::quicksight::GetThemeConfigurationSheet>,
    /// Determines the typography options. See typography.
    #[builder(into)]
    #[serde(rename = "typographies")]
    pub r#typographies: Vec<super::super::types::quicksight::GetThemeConfigurationTypography>,
    /// Color properties that apply to the UI and to charts, excluding the colors that apply to data. See ui_color_palette.
    #[builder(into)]
    #[serde(rename = "uiColorPalettes")]
    pub r#ui_color_palettes: Vec<super::super::types::quicksight::GetThemeConfigurationUiColorPalette>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetThemeConfiguration {
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
                "data_color_palettes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_color_palettes,
                )
                .await,
            );
            map.insert(
                "sheets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sheets,
                )
                .await,
            );
            map.insert(
                "typographies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#typographies,
                )
                .await,
            );
            map.insert(
                "ui_color_palettes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ui_color_palettes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetThemeConfiguration {
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
                    r#data_color_palettes: {
                        let field_value = match fields_map.get("data_color_palettes") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_color_palettes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sheets: {
                        let field_value = match fields_map.get("sheets") {
                            Some(value) => value,
                            None => bail!("Missing field 'sheets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#typographies: {
                        let field_value = match fields_map.get("typographies") {
                            Some(value) => value,
                            None => bail!("Missing field 'typographies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ui_color_palettes: {
                        let field_value = match fields_map.get("ui_color_palettes") {
                            Some(value) => value,
                            None => bail!("Missing field 'ui_color_palettes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
