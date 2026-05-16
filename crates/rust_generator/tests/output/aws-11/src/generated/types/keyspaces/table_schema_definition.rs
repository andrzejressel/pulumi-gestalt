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

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("clustering_keys".to_string(), self.r#clustering_keys.to_pulumi_value().await);
            map.insert("columns".to_string(), self.r#columns.to_pulumi_value().await);
            map.insert("partition_keys".to_string(), self.r#partition_keys.to_pulumi_value().await);
            map.insert("static_columns".to_string(), self.r#static_columns.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableSchemaDefinition {
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
                    r#clustering_keys: {
                        let field_value = match fields_map.get("clustering_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'clustering_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::keyspaces::TableSchemaDefinitionClusteringKey>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#columns: {
                        let field_value = match fields_map.get("columns") {
                            Some(value) => value,
                            None => bail!("Missing field 'columns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::keyspaces::TableSchemaDefinitionColumn> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#partition_keys: {
                        let field_value = match fields_map.get("partition_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'partition_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::keyspaces::TableSchemaDefinitionPartitionKey> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#static_columns: {
                        let field_value = match fields_map.get("static_columns") {
                            Some(value) => value,
                            None => bail!("Missing field 'static_columns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::keyspaces::TableSchemaDefinitionStaticColumn>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
