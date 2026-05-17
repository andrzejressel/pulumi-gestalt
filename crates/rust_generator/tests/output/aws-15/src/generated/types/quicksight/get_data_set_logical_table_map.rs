#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSetLogicalTableMap {
    #[builder(into)]
    #[serde(rename = "alias")]
    pub r#alias: String,
    #[builder(into)]
    #[serde(rename = "dataTransforms")]
    pub r#data_transforms: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransform>,
    #[builder(into)]
    #[serde(rename = "logicalTableMapId")]
    pub r#logical_table_map_id: String,
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapSource>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataSetLogicalTableMap {
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
                "sources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sources,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataSetLogicalTableMap {
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
                    r#sources: {
                        let field_value = match fields_map.get("sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
