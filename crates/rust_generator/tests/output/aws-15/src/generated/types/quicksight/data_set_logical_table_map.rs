#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetLogicalTableMap {
    /// A display name for the logical table.
    #[builder(into)]
    #[serde(rename = "alias")]
    pub r#alias: String,
    /// Transform operations that act on this logical table. For this structure to be valid, only one of the attributes can be non-null. See data_transforms.
    #[builder(into)]
    #[serde(rename = "dataTransforms")]
    pub r#data_transforms: Option<Vec<super::super::types::quicksight::DataSetLogicalTableMapDataTransform>>,
    /// Key of the logical table map.
    #[builder(into)]
    #[serde(rename = "logicalTableMapId")]
    pub r#logical_table_map_id: String,
    /// Source of this logical table. See source.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<super::super::types::quicksight::DataSetLogicalTableMapSource>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSetLogicalTableMap {
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
                "alias".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#alias,
                )
                .await,
            );
            map.insert(
                "data_transforms".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_transforms,
                )
                .await,
            );
            map.insert(
                "logical_table_map_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#logical_table_map_id,
                )
                .await,
            );
            map.insert(
                "source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSetLogicalTableMap {
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
                    r#alias: {
                        let field_value = match fields_map.get("alias") {
                            Some(value) => value,
                            None => bail!("Missing field 'alias' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_transforms: {
                        let field_value = match fields_map.get("data_transforms") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_transforms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logical_table_map_id: {
                        let field_value = match fields_map.get("logical_table_map_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'logical_table_map_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source: {
                        let field_value = match fields_map.get("source") {
                            Some(value) => value,
                            None => bail!("Missing field 'source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
