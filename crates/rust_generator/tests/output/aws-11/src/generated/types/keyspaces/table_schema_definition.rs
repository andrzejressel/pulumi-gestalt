#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableSchemaDefinition {
    /// The columns that are part of the clustering key of the table.
    #[builder(into)]
    #[serde(rename = "clusteringKeys")]
    pub r#clustering_keys: Option<Vec<super::super::types::keyspaces::TableSchemaDefinitionClusteringKey>>,
    /// The regular columns of the table.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Vec<super::super::types::keyspaces::TableSchemaDefinitionColumn>,
    /// The columns that are part of the partition key of the table .
    #[builder(into)]
    #[serde(rename = "partitionKeys")]
    pub r#partition_keys: Vec<super::super::types::keyspaces::TableSchemaDefinitionPartitionKey>,
    /// The columns that have been defined as `STATIC`. Static columns store values that are shared by all rows in the same partition.
    #[builder(into)]
    #[serde(rename = "staticColumns")]
    pub r#static_columns: Option<Vec<super::super::types::keyspaces::TableSchemaDefinitionStaticColumn>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableSchemaDefinition {
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
                "clustering_keys".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#clustering_keys,
                )
                .await,
            );
            map.insert(
                "columns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#columns,
                )
                .await,
            );
            map.insert(
                "partition_keys".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#partition_keys,
                )
                .await,
            );
            map.insert(
                "static_columns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#static_columns,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableSchemaDefinition {
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
                    r#clustering_keys: {
                        let field_value = match fields_map.get("clustering_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'clustering_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#columns: {
                        let field_value = match fields_map.get("columns") {
                            Some(value) => value,
                            None => bail!("Missing field 'columns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#partition_keys: {
                        let field_value = match fields_map.get("partition_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'partition_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#static_columns: {
                        let field_value = match fields_map.get("static_columns") {
                            Some(value) => value,
                            None => bail!("Missing field 'static_columns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
