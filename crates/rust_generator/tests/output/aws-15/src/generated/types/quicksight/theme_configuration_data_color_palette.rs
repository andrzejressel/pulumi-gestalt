#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThemeConfigurationDataColorPalette {
    /// List of hexadecimal codes for the colors. Minimum of 8 items and maximum of 20 items.
    #[builder(into)]
    #[serde(rename = "colors")]
    pub r#colors: Option<Vec<String>>,
    /// The hexadecimal code of a color that applies to charts where a lack of data is highlighted.
    #[builder(into)]
    #[serde(rename = "emptyFillColor")]
    pub r#empty_fill_color: Option<String>,
    /// The minimum and maximum hexadecimal codes that describe a color gradient. List of exactly 2 items.
    #[builder(into)]
    #[serde(rename = "minMaxGradients")]
    pub r#min_max_gradients: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ThemeConfigurationDataColorPalette {
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
                "colors".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#colors,
                )
                .await,
            );
            map.insert(
                "empty_fill_color".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#empty_fill_color,
                )
                .await,
            );
            map.insert(
                "min_max_gradients".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_max_gradients,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ThemeConfigurationDataColorPalette {
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
                    r#colors: {
                        let field_value = match fields_map.get("colors") {
                            Some(value) => value,
                            None => bail!("Missing field 'colors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#empty_fill_color: {
                        let field_value = match fields_map.get("empty_fill_color") {
                            Some(value) => value,
                            None => bail!("Missing field 'empty_fill_color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_max_gradients: {
                        let field_value = match fields_map.get("min_max_gradients") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_max_gradients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
