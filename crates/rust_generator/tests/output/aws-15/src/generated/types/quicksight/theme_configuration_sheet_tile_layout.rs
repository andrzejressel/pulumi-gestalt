#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThemeConfigurationSheetTileLayout {
    /// The gutter settings that apply between tiles. See gutter.
    #[builder(into)]
    #[serde(rename = "gutter")]
    pub r#gutter: Option<Box<super::super::types::quicksight::ThemeConfigurationSheetTileLayoutGutter>>,
    /// The margin settings that apply around the outside edge of sheets. See margin.
    #[builder(into)]
    #[serde(rename = "margin")]
    pub r#margin: Option<Box<super::super::types::quicksight::ThemeConfigurationSheetTileLayoutMargin>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ThemeConfigurationSheetTileLayout {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("gutter".to_string(), self.r#gutter.to_pulumi_value().await);
            map.insert("margin".to_string(), self.r#margin.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ThemeConfigurationSheetTileLayout {
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
                    r#gutter: {
                        let field_value = match fields_map.get("gutter") {
                            Some(value) => value,
                            None => bail!("Missing field 'gutter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::ThemeConfigurationSheetTileLayoutGutter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#margin: {
                        let field_value = match fields_map.get("margin") {
                            Some(value) => value,
                            None => bail!("Missing field 'margin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::ThemeConfigurationSheetTileLayoutMargin>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
