#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSetPhysicalTableMap {
    #[builder(into)]
    #[serde(rename = "customSqls")]
    pub r#custom_sqls: Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapCustomSql>,
    #[builder(into)]
    #[serde(rename = "physicalTableMapId")]
    pub r#physical_table_map_id: String,
    #[builder(into)]
    #[serde(rename = "relationalTables")]
    pub r#relational_tables: Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapRelationalTable>,
    #[builder(into)]
    #[serde(rename = "s3Sources")]
    pub r#s_3_sources: Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapS3Source>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataSetPhysicalTableMap {
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
                "custom_sqls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_sqls,
                )
                .await,
            );
            map.insert(
                "physical_table_map_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#physical_table_map_id,
                )
                .await,
            );
            map.insert(
                "relational_tables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#relational_tables,
                )
                .await,
            );
            map.insert(
                "s_3_sources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_sources,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataSetPhysicalTableMap {
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
                    r#custom_sqls: {
                        let field_value = match fields_map.get("custom_sqls") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_sqls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#physical_table_map_id: {
                        let field_value = match fields_map.get("physical_table_map_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'physical_table_map_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#relational_tables: {
                        let field_value = match fields_map.get("relational_tables") {
                            Some(value) => value,
                            None => bail!("Missing field 'relational_tables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_sources: {
                        let field_value = match fields_map.get("s_3_sources") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
