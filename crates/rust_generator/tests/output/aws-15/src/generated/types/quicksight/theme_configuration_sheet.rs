#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThemeConfigurationSheet {
    /// The display options for tiles. See tile.
    #[builder(into)]
    #[serde(rename = "tile")]
    pub r#tile: Option<Box<super::super::types::quicksight::ThemeConfigurationSheetTile>>,
    /// The layout options for tiles. See tile_layout.
    #[builder(into)]
    #[serde(rename = "tileLayout")]
    pub r#tile_layout: Option<Box<super::super::types::quicksight::ThemeConfigurationSheetTileLayout>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ThemeConfigurationSheet {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("tile".to_string(), self.r#tile.to_pulumi_value().await);
            map.insert("tile_layout".to_string(), self.r#tile_layout.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ThemeConfigurationSheet {
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
                    r#tile: {
                        let field_value = match fields_map.get("tile") {
                            Some(value) => value,
                            None => bail!("Missing field 'tile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::ThemeConfigurationSheetTile>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tile_layout: {
                        let field_value = match fields_map.get("tile_layout") {
                            Some(value) => value,
                            None => bail!("Missing field 'tile_layout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::ThemeConfigurationSheetTileLayout>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
