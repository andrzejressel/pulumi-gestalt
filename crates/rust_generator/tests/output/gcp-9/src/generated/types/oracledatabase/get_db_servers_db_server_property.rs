#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDbServersDbServerProperty {
    /// Output only
    #[builder(into)]
    #[serde(rename = "dbNodeIds")]
    pub r#db_node_ids: Vec<String>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeGb")]
    pub r#db_node_storage_size_gb: i32,
    /// Output only
    #[builder(into)]
    #[serde(rename = "maxDbNodeStorageSizeGb")]
    pub r#max_db_node_storage_size_gb: i32,
    /// Output only
    #[builder(into)]
    #[serde(rename = "maxMemorySizeGb")]
    pub r#max_memory_size_gb: i32,
    /// Output only
    #[builder(into)]
    #[serde(rename = "maxOcpuCount")]
    pub r#max_ocpu_count: i32,
    /// Output only
    #[builder(into)]
    #[serde(rename = "memorySizeGb")]
    pub r#memory_size_gb: i32,
    /// Output only
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: String,
    /// Output only
    #[builder(into)]
    #[serde(rename = "ocpuCount")]
    pub r#ocpu_count: i32,
    /// Output only
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
    /// Output only
    #[builder(into)]
    #[serde(rename = "vmCount")]
    pub r#vm_count: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDbServersDbServerProperty {
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
                "db_node_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#db_node_ids,
                )
                .await,
            );
            map.insert(
                "db_node_storage_size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#db_node_storage_size_gb,
                )
                .await,
            );
            map.insert(
                "max_db_node_storage_size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_db_node_storage_size_gb,
                )
                .await,
            );
            map.insert(
                "max_memory_size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_memory_size_gb,
                )
                .await,
            );
            map.insert(
                "max_ocpu_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_ocpu_count,
                )
                .await,
            );
            map.insert(
                "memory_size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#memory_size_gb,
                )
                .await,
            );
            map.insert(
                "ocid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ocid,
                )
                .await,
            );
            map.insert(
                "ocpu_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ocpu_count,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
                )
                .await,
            );
            map.insert(
                "vm_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vm_count,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDbServersDbServerProperty {
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
                    r#db_node_ids: {
                        let field_value = match fields_map.get("db_node_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_node_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_node_storage_size_gb: {
                        let field_value = match fields_map.get("db_node_storage_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_node_storage_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_db_node_storage_size_gb: {
                        let field_value = match fields_map.get("max_db_node_storage_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_db_node_storage_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_memory_size_gb: {
                        let field_value = match fields_map.get("max_memory_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_memory_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_ocpu_count: {
                        let field_value = match fields_map.get("max_ocpu_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_ocpu_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_size_gb: {
                        let field_value = match fields_map.get("memory_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ocid: {
                        let field_value = match fields_map.get("ocid") {
                            Some(value) => value,
                            None => bail!("Missing field 'ocid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ocpu_count: {
                        let field_value = match fields_map.get("ocpu_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'ocpu_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_count: {
                        let field_value = match fields_map.get("vm_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
