#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDbSystemShapesDbSystemShape {
    /// The maximum number of CPU cores that can be enabled on the DB system for this shape.
    #[builder(into)]
    #[serde(rename = "availableCoreCount")]
    pub r#available_core_count: i32,
    /// The maximum number of CPU cores per database node that can be enabled for this shape. Only applicable to the flex Exadata shape, ExaCC Elastic shapes and VM Flex shapes.
    #[builder(into)]
    #[serde(rename = "availableCoreCountPerNode")]
    pub r#available_core_count_per_node: i32,
    /// The maximum data storage that can be enabled for this shape.
    #[builder(into)]
    #[serde(rename = "availableDataStorageInTbs")]
    pub r#available_data_storage_in_tbs: i32,
    /// The maximum data storage available per storage server for this shape. Only applicable to ExaCC Elastic shapes.
    #[builder(into)]
    #[serde(rename = "availableDataStoragePerServerInTbs")]
    pub r#available_data_storage_per_server_in_tbs: i32,
    /// The maximum DB Node storage available per database node for this shape. Only applicable to ExaCC Elastic shapes.
    #[builder(into)]
    #[serde(rename = "availableDbNodePerNodeInGbs")]
    pub r#available_db_node_per_node_in_gbs: i32,
    /// The maximum DB Node storage that can be enabled for this shape.
    #[builder(into)]
    #[serde(rename = "availableDbNodeStorageInGbs")]
    pub r#available_db_node_storage_in_gbs: i32,
    /// The maximum memory that can be enabled for this shape.
    #[builder(into)]
    #[serde(rename = "availableMemoryInGbs")]
    pub r#available_memory_in_gbs: i32,
    /// The maximum memory available per database node for this shape. Only applicable to ExaCC Elastic shapes.
    #[builder(into)]
    #[serde(rename = "availableMemoryPerNodeInGbs")]
    pub r#available_memory_per_node_in_gbs: i32,
    /// The discrete number by which the CPU core count for this shape can be increased or decreased.
    #[builder(into)]
    #[serde(rename = "coreCountIncrement")]
    pub r#core_count_increment: i32,
    /// The maximum number of compute servers available for this shape.
    #[builder(into)]
    #[serde(rename = "maximumNodeCount")]
    pub r#maximum_node_count: i32,
    /// The maximum number of Exadata storage servers available for the Exadata infrastructure.
    #[builder(into)]
    #[serde(rename = "maximumStorageCount")]
    pub r#maximum_storage_count: i32,
    /// The minimum number of CPU cores that can be enabled on the DB system for this shape.
    #[builder(into)]
    #[serde(rename = "minimumCoreCount")]
    pub r#minimum_core_count: i32,
    /// The minimum number of CPU cores that can be enabled per node for this shape.
    #[builder(into)]
    #[serde(rename = "minimumCoreCountPerNode")]
    pub r#minimum_core_count_per_node: i32,
    /// The minimum data storage that need be allocated for this shape.
    #[builder(into)]
    #[serde(rename = "minimumDataStorageInTbs")]
    pub r#minimum_data_storage_in_tbs: i32,
    /// The minimum DB Node storage that need be allocated per node for this shape.
    #[builder(into)]
    #[serde(rename = "minimumDbNodeStoragePerNodeInGbs")]
    pub r#minimum_db_node_storage_per_node_in_gbs: i32,
    /// The minimum memory that need be allocated per node for this shape.
    #[builder(into)]
    #[serde(rename = "minimumMemoryPerNodeInGbs")]
    pub r#minimum_memory_per_node_in_gbs: i32,
    /// The minimum number of compute servers available for this shape.
    #[builder(into)]
    #[serde(rename = "minimumNodeCount")]
    pub r#minimum_node_count: i32,
    /// The minimum number of Exadata storage servers available for the Exadata infrastructure.
    #[builder(into)]
    #[serde(rename = "minimumStorageCount")]
    pub r#minimum_storage_count: i32,
    /// The runtime minimum number of compute servers available for this shape.
    #[builder(into)]
    #[serde(rename = "runtimeMinimumCoreCount")]
    pub r#runtime_minimum_core_count: i32,
    /// The family of the shape used for the DB system.
    #[builder(into)]
    #[serde(rename = "shapeFamily")]
    pub r#shape_family: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDbSystemShapesDbSystemShape {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "available_core_count",
                    &self.r#available_core_count,
                ),
                to_pulumi_object_field(
                    "available_core_count_per_node",
                    &self.r#available_core_count_per_node,
                ),
                to_pulumi_object_field(
                    "available_data_storage_in_tbs",
                    &self.r#available_data_storage_in_tbs,
                ),
                to_pulumi_object_field(
                    "available_data_storage_per_server_in_tbs",
                    &self.r#available_data_storage_per_server_in_tbs,
                ),
                to_pulumi_object_field(
                    "available_db_node_per_node_in_gbs",
                    &self.r#available_db_node_per_node_in_gbs,
                ),
                to_pulumi_object_field(
                    "available_db_node_storage_in_gbs",
                    &self.r#available_db_node_storage_in_gbs,
                ),
                to_pulumi_object_field(
                    "available_memory_in_gbs",
                    &self.r#available_memory_in_gbs,
                ),
                to_pulumi_object_field(
                    "available_memory_per_node_in_gbs",
                    &self.r#available_memory_per_node_in_gbs,
                ),
                to_pulumi_object_field(
                    "core_count_increment",
                    &self.r#core_count_increment,
                ),
                to_pulumi_object_field(
                    "maximum_node_count",
                    &self.r#maximum_node_count,
                ),
                to_pulumi_object_field(
                    "maximum_storage_count",
                    &self.r#maximum_storage_count,
                ),
                to_pulumi_object_field(
                    "minimum_core_count",
                    &self.r#minimum_core_count,
                ),
                to_pulumi_object_field(
                    "minimum_core_count_per_node",
                    &self.r#minimum_core_count_per_node,
                ),
                to_pulumi_object_field(
                    "minimum_data_storage_in_tbs",
                    &self.r#minimum_data_storage_in_tbs,
                ),
                to_pulumi_object_field(
                    "minimum_db_node_storage_per_node_in_gbs",
                    &self.r#minimum_db_node_storage_per_node_in_gbs,
                ),
                to_pulumi_object_field(
                    "minimum_memory_per_node_in_gbs",
                    &self.r#minimum_memory_per_node_in_gbs,
                ),
                to_pulumi_object_field(
                    "minimum_node_count",
                    &self.r#minimum_node_count,
                ),
                to_pulumi_object_field(
                    "minimum_storage_count",
                    &self.r#minimum_storage_count,
                ),
                to_pulumi_object_field(
                    "runtime_minimum_core_count",
                    &self.r#runtime_minimum_core_count,
                ),
                to_pulumi_object_field(
                    "shape_family",
                    &self.r#shape_family,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDbSystemShapesDbSystemShape {
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
                    r#available_core_count: {
                        let field_value = match fields_map.get("available_core_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_core_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_core_count_per_node: {
                        let field_value = match fields_map.get("available_core_count_per_node") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_core_count_per_node' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_data_storage_in_tbs: {
                        let field_value = match fields_map.get("available_data_storage_in_tbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_data_storage_in_tbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_data_storage_per_server_in_tbs: {
                        let field_value = match fields_map.get("available_data_storage_per_server_in_tbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_data_storage_per_server_in_tbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_db_node_per_node_in_gbs: {
                        let field_value = match fields_map.get("available_db_node_per_node_in_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_db_node_per_node_in_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_db_node_storage_in_gbs: {
                        let field_value = match fields_map.get("available_db_node_storage_in_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_db_node_storage_in_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_memory_in_gbs: {
                        let field_value = match fields_map.get("available_memory_in_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_memory_in_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_memory_per_node_in_gbs: {
                        let field_value = match fields_map.get("available_memory_per_node_in_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_memory_per_node_in_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#core_count_increment: {
                        let field_value = match fields_map.get("core_count_increment") {
                            Some(value) => value,
                            None => bail!("Missing field 'core_count_increment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_node_count: {
                        let field_value = match fields_map.get("maximum_node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_storage_count: {
                        let field_value = match fields_map.get("maximum_storage_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_storage_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_core_count: {
                        let field_value = match fields_map.get("minimum_core_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_core_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_core_count_per_node: {
                        let field_value = match fields_map.get("minimum_core_count_per_node") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_core_count_per_node' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_data_storage_in_tbs: {
                        let field_value = match fields_map.get("minimum_data_storage_in_tbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_data_storage_in_tbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_db_node_storage_per_node_in_gbs: {
                        let field_value = match fields_map.get("minimum_db_node_storage_per_node_in_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_db_node_storage_per_node_in_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_memory_per_node_in_gbs: {
                        let field_value = match fields_map.get("minimum_memory_per_node_in_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_memory_per_node_in_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_node_count: {
                        let field_value = match fields_map.get("minimum_node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_storage_count: {
                        let field_value = match fields_map.get("minimum_storage_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_storage_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#runtime_minimum_core_count: {
                        let field_value = match fields_map.get("runtime_minimum_core_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtime_minimum_core_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shape_family: {
                        let field_value = match fields_map.get("shape_family") {
                            Some(value) => value,
                            None => bail!("Missing field 'shape_family' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
