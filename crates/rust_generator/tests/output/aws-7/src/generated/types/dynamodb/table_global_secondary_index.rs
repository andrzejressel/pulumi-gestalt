#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableGlobalSecondaryIndex {
    /// Name of the hash key in the index; must be defined as an attribute in the resource.
    #[builder(into)]
    #[serde(rename = "hashKey")]
    pub r#hash_key: String,
    /// Name of the index.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Only required with `INCLUDE` as a projection type; a list of attributes to project into the index. These do not need to be defined as attributes on the table.
    #[builder(into)]
    #[serde(rename = "nonKeyAttributes")]
    pub r#non_key_attributes: Option<Vec<String>>,
    /// Sets the maximum number of read and write units for the specified on-demand table. See below.
    #[builder(into)]
    #[serde(rename = "onDemandThroughput")]
    pub r#on_demand_throughput: Option<Box<super::super::types::dynamodb::TableGlobalSecondaryIndexOnDemandThroughput>>,
    /// One of `ALL`, `INCLUDE` or `KEYS_ONLY` where `ALL` projects every attribute into the index, `KEYS_ONLY` projects  into the index only the table and index hash_key and sort_key attributes ,  `INCLUDE` projects into the index all of the attributes that are defined in `non_key_attributes` in addition to the attributes that that`KEYS_ONLY` project.
    #[builder(into)]
    #[serde(rename = "projectionType")]
    pub r#projection_type: String,
    /// Name of the range key; must be defined
    #[builder(into)]
    #[serde(rename = "rangeKey")]
    pub r#range_key: Option<String>,
    /// Number of read units for this index. Must be set if billing_mode is set to PROVISIONED.
    #[builder(into)]
    #[serde(rename = "readCapacity")]
    pub r#read_capacity: Option<i32>,
    /// Number of write units for this index. Must be set if billing_mode is set to PROVISIONED.
    #[builder(into)]
    #[serde(rename = "writeCapacity")]
    pub r#write_capacity: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableGlobalSecondaryIndex {
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
                "hash_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hash_key,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "non_key_attributes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#non_key_attributes,
                )
                .await,
            );
            map.insert(
                "on_demand_throughput".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#on_demand_throughput,
                )
                .await,
            );
            map.insert(
                "projection_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#projection_type,
                )
                .await,
            );
            map.insert(
                "range_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#range_key,
                )
                .await,
            );
            map.insert(
                "read_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#read_capacity,
                )
                .await,
            );
            map.insert(
                "write_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#write_capacity,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableGlobalSecondaryIndex {
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
                    r#hash_key: {
                        let field_value = match fields_map.get("hash_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'hash_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#non_key_attributes: {
                        let field_value = match fields_map.get("non_key_attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'non_key_attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_demand_throughput: {
                        let field_value = match fields_map.get("on_demand_throughput") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_demand_throughput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#projection_type: {
                        let field_value = match fields_map.get("projection_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'projection_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#range_key: {
                        let field_value = match fields_map.get("range_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'range_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_capacity: {
                        let field_value = match fields_map.get("read_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_capacity: {
                        let field_value = match fields_map.get("write_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'write_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
