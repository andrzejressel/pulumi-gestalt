#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetPhysicalTableMap {
    /// A physical table type built from the results of the custom SQL query. See custom_sql.
    #[builder(into)]
    pub r#custom_sql: Option<Box<super::super::types::quicksight::DataSetPhysicalTableMapCustomSql>>,
    /// Key of the physical table map.
    #[builder(into)]
    pub r#physical_table_map_id: String,
    /// A physical table type for relational data sources. See relational_table.
    #[builder(into)]
    pub r#relational_table: Option<Box<super::super::types::quicksight::DataSetPhysicalTableMapRelationalTable>>,
    /// A physical table type for as S3 data source. See s3_source.
    #[builder(into)]
    pub r#s_3_source: Option<Box<super::super::types::quicksight::DataSetPhysicalTableMapS3Source>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSetPhysicalTableMap {
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
                    "customSql",
                    &self.r#custom_sql,
                ),
                to_pulumi_object_field(
                    "physicalTableMapId",
                    &self.r#physical_table_map_id,
                ),
                to_pulumi_object_field(
                    "relationalTable",
                    &self.r#relational_table,
                ),
                to_pulumi_object_field(
                    "s3Source",
                    &self.r#s_3_source,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSetPhysicalTableMap {
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
                    r#custom_sql: {
                        let field_value = match fields_map.get("customSql") {
                            Some(value) => value,
                            None => bail!("Missing field 'customSql' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#physical_table_map_id: {
                        let field_value = match fields_map.get("physicalTableMapId") {
                            Some(value) => value,
                            None => bail!("Missing field 'physicalTableMapId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#relational_table: {
                        let field_value = match fields_map.get("relationalTable") {
                            Some(value) => value,
                            None => bail!("Missing field 'relationalTable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_source: {
                        let field_value = match fields_map.get("s3Source") {
                            Some(value) => value,
                            None => bail!("Missing field 's3Source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
