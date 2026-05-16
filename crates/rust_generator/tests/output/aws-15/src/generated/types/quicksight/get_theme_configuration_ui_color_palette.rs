#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetThemeConfigurationUiColorPalette {
    /// Color (hexadecimal) that applies to selected states and buttons.
    #[builder(into)]
    #[serde(rename = "accent")]
    pub r#accent: String,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the accent color.
    #[builder(into)]
    #[serde(rename = "accentForeground")]
    pub r#accent_foreground: String,
    /// Color (hexadecimal) that applies to error messages.
    #[builder(into)]
    #[serde(rename = "danger")]
    pub r#danger: String,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the error color.
    #[builder(into)]
    #[serde(rename = "dangerForeground")]
    pub r#danger_foreground: String,
    /// Color (hexadecimal) that applies to the names of fields that are identified as dimensions.
    #[builder(into)]
    #[serde(rename = "dimension")]
    pub r#dimension: String,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the dimension color.
    #[builder(into)]
    #[serde(rename = "dimensionForeground")]
    pub r#dimension_foreground: String,
    /// Color (hexadecimal) that applies to the names of fields that are identified as measures.
    #[builder(into)]
    #[serde(rename = "measure")]
    pub r#measure: String,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the measure color.
    #[builder(into)]
    #[serde(rename = "measureForeground")]
    pub r#measure_foreground: String,
    /// Color (hexadecimal) that applies to visuals and other high emphasis UI.
    #[builder(into)]
    #[serde(rename = "primaryBackground")]
    pub r#primary_background: String,
    /// Color (hexadecimal) of text and other foreground elements that appear over the primary background regions, such as grid lines, borders, table banding, icons, and so on.
    #[builder(into)]
    #[serde(rename = "primaryForeground")]
    pub r#primary_foreground: String,
    /// Color (hexadecimal) that applies to the sheet background and sheet controls.
    #[builder(into)]
    #[serde(rename = "secondaryBackground")]
    pub r#secondary_background: String,
    /// Color (hexadecimal) that applies to any sheet title, sheet control text, or UI that appears over the secondary background.
    #[builder(into)]
    #[serde(rename = "secondaryForeground")]
    pub r#secondary_foreground: String,
    /// Color (hexadecimal) that applies to success messages, for example the check mark for a successful download.
    #[builder(into)]
    #[serde(rename = "success")]
    pub r#success: String,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the success color.
    #[builder(into)]
    #[serde(rename = "successForeground")]
    pub r#success_foreground: String,
    /// Color (hexadecimal) that applies to warning and informational messages.
    #[builder(into)]
    #[serde(rename = "warning")]
    pub r#warning: String,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the warning color.
    #[builder(into)]
    #[serde(rename = "warningForeground")]
    pub r#warning_foreground: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetThemeConfigurationUiColorPalette {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("accent".to_string(), self.r#accent.to_pulumi_value().await);
            map.insert("accent_foreground".to_string(), self.r#accent_foreground.to_pulumi_value().await);
            map.insert("danger".to_string(), self.r#danger.to_pulumi_value().await);
            map.insert("danger_foreground".to_string(), self.r#danger_foreground.to_pulumi_value().await);
            map.insert("dimension".to_string(), self.r#dimension.to_pulumi_value().await);
            map.insert("dimension_foreground".to_string(), self.r#dimension_foreground.to_pulumi_value().await);
            map.insert("measure".to_string(), self.r#measure.to_pulumi_value().await);
            map.insert("measure_foreground".to_string(), self.r#measure_foreground.to_pulumi_value().await);
            map.insert("primary_background".to_string(), self.r#primary_background.to_pulumi_value().await);
            map.insert("primary_foreground".to_string(), self.r#primary_foreground.to_pulumi_value().await);
            map.insert("secondary_background".to_string(), self.r#secondary_background.to_pulumi_value().await);
            map.insert("secondary_foreground".to_string(), self.r#secondary_foreground.to_pulumi_value().await);
            map.insert("success".to_string(), self.r#success.to_pulumi_value().await);
            map.insert("success_foreground".to_string(), self.r#success_foreground.to_pulumi_value().await);
            map.insert("warning".to_string(), self.r#warning.to_pulumi_value().await);
            map.insert("warning_foreground".to_string(), self.r#warning_foreground.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetThemeConfigurationUiColorPalette {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#accent: {
                        let field_value = match fields_map.get("accent") {
                            Some(value) => value,
                            None => bail!("Missing field 'accent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#accent_foreground: {
                        let field_value = match fields_map.get("accent_foreground") {
                            Some(value) => value,
                            None => bail!("Missing field 'accent_foreground' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#danger: {
                        let field_value = match fields_map.get("danger") {
                            Some(value) => value,
                            None => bail!("Missing field 'danger' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#danger_foreground: {
                        let field_value = match fields_map.get("danger_foreground") {
                            Some(value) => value,
                            None => bail!("Missing field 'danger_foreground' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dimension: {
                        let field_value = match fields_map.get("dimension") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimension' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dimension_foreground: {
                        let field_value = match fields_map.get("dimension_foreground") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimension_foreground' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#measure: {
                        let field_value = match fields_map.get("measure") {
                            Some(value) => value,
                            None => bail!("Missing field 'measure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#measure_foreground: {
                        let field_value = match fields_map.get("measure_foreground") {
                            Some(value) => value,
                            None => bail!("Missing field 'measure_foreground' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#primary_background: {
                        let field_value = match fields_map.get("primary_background") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_background' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#primary_foreground: {
                        let field_value = match fields_map.get("primary_foreground") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_foreground' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#secondary_background: {
                        let field_value = match fields_map.get("secondary_background") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_background' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#secondary_foreground: {
                        let field_value = match fields_map.get("secondary_foreground") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_foreground' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#success: {
                        let field_value = match fields_map.get("success") {
                            Some(value) => value,
                            None => bail!("Missing field 'success' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#success_foreground: {
                        let field_value = match fields_map.get("success_foreground") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_foreground' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#warning: {
                        let field_value = match fields_map.get("warning") {
                            Some(value) => value,
                            None => bail!("Missing field 'warning' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#warning_foreground: {
                        let field_value = match fields_map.get("warning_foreground") {
                            Some(value) => value,
                            None => bail!("Missing field 'warning_foreground' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
