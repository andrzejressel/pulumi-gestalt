#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSetLogicalTableMapDataTransform {
    #[builder(into)]
    #[serde(rename = "castColumnTypeOperations")]
    pub r#cast_column_type_operations: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformCastColumnTypeOperation>,
    #[builder(into)]
    #[serde(rename = "createColumnsOperations")]
    pub r#create_columns_operations: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformCreateColumnsOperation>,
    #[builder(into)]
    #[serde(rename = "filterOperations")]
    pub r#filter_operations: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformFilterOperation>,
    #[builder(into)]
    #[serde(rename = "projectOperations")]
    pub r#project_operations: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformProjectOperation>,
    #[builder(into)]
    #[serde(rename = "renameColumnOperations")]
    pub r#rename_column_operations: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformRenameColumnOperation>,
    #[builder(into)]
    #[serde(rename = "tagColumnOperations")]
    pub r#tag_column_operations: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformTagColumnOperation>,
    #[builder(into)]
    #[serde(rename = "untagColumnOperations")]
    pub r#untag_column_operations: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformUntagColumnOperation>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataSetLogicalTableMapDataTransform {
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
                "cast_column_type_operations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cast_column_type_operations,
                )
                .await,
            );
            map.insert(
                "create_columns_operations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#create_columns_operations,
                )
                .await,
            );
            map.insert(
                "filter_operations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filter_operations,
                )
                .await,
            );
            map.insert(
                "project_operations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#project_operations,
                )
                .await,
            );
            map.insert(
                "rename_column_operations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rename_column_operations,
                )
                .await,
            );
            map.insert(
                "tag_column_operations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_column_operations,
                )
                .await,
            );
            map.insert(
                "untag_column_operations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#untag_column_operations,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataSetLogicalTableMapDataTransform {
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
                    r#cast_column_type_operations: {
                        let field_value = match fields_map.get("cast_column_type_operations") {
                            Some(value) => value,
                            None => bail!("Missing field 'cast_column_type_operations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_columns_operations: {
                        let field_value = match fields_map.get("create_columns_operations") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_columns_operations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_operations: {
                        let field_value = match fields_map.get("filter_operations") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_operations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project_operations: {
                        let field_value = match fields_map.get("project_operations") {
                            Some(value) => value,
                            None => bail!("Missing field 'project_operations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rename_column_operations: {
                        let field_value = match fields_map.get("rename_column_operations") {
                            Some(value) => value,
                            None => bail!("Missing field 'rename_column_operations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_column_operations: {
                        let field_value = match fields_map.get("tag_column_operations") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_column_operations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#untag_column_operations: {
                        let field_value = match fields_map.get("untag_column_operations") {
                            Some(value) => value,
                            None => bail!("Missing field 'untag_column_operations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
