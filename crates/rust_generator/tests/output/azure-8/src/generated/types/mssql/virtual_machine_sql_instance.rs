#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineSqlInstance {
    /// Specifies if the SQL Server is optimized for adhoc workloads. Possible values are `true` and `false`. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "adhocWorkloadsOptimizationEnabled")]
    pub r#adhoc_workloads_optimization_enabled: Option<bool>,
    /// Collation of the SQL Server. Defaults to `SQL_Latin1_General_CP1_CI_AS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "collation")]
    pub r#collation: Option<String>,
    /// Specifies if Instant File Initialization is enabled for the SQL Server. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "instantFileInitializationEnabled")]
    pub r#instant_file_initialization_enabled: Option<bool>,
    /// Specifies if Lock Pages in Memory is enabled for the SQL Server. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "lockPagesInMemoryEnabled")]
    pub r#lock_pages_in_memory_enabled: Option<bool>,
    /// Maximum Degree of Parallelism of the SQL Server. Possible values are between `0` and `32767`. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "maxDop")]
    pub r#max_dop: Option<i32>,
    /// Maximum amount memory that SQL Server Memory Manager can allocate to the SQL Server process. Possible values are between `128` and `2147483647` Defaults to `2147483647`.
    #[builder(into)]
    #[serde(rename = "maxServerMemoryMb")]
    pub r#max_server_memory_mb: Option<i32>,
    /// Minimum amount memory that SQL Server Memory Manager can allocate to the SQL Server process. Possible values are between `0` and `2147483647` Defaults to `0`.
    /// 
    /// > **NOTE:** `max_server_memory_mb` must be greater than or equal to `min_server_memory_mb`
    #[builder(into)]
    #[serde(rename = "minServerMemoryMb")]
    pub r#min_server_memory_mb: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineSqlInstance {
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
                "adhoc_workloads_optimization_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#adhoc_workloads_optimization_enabled,
                )
                .await,
            );
            map.insert(
                "collation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#collation,
                )
                .await,
            );
            map.insert(
                "instant_file_initialization_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instant_file_initialization_enabled,
                )
                .await,
            );
            map.insert(
                "lock_pages_in_memory_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lock_pages_in_memory_enabled,
                )
                .await,
            );
            map.insert(
                "max_dop".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_dop,
                )
                .await,
            );
            map.insert(
                "max_server_memory_mb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_server_memory_mb,
                )
                .await,
            );
            map.insert(
                "min_server_memory_mb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_server_memory_mb,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineSqlInstance {
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
                    r#adhoc_workloads_optimization_enabled: {
                        let field_value = match fields_map.get("adhoc_workloads_optimization_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'adhoc_workloads_optimization_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#collation: {
                        let field_value = match fields_map.get("collation") {
                            Some(value) => value,
                            None => bail!("Missing field 'collation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instant_file_initialization_enabled: {
                        let field_value = match fields_map.get("instant_file_initialization_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'instant_file_initialization_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lock_pages_in_memory_enabled: {
                        let field_value = match fields_map.get("lock_pages_in_memory_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'lock_pages_in_memory_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_dop: {
                        let field_value = match fields_map.get("max_dop") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_dop' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_server_memory_mb: {
                        let field_value = match fields_map.get("max_server_memory_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_server_memory_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_server_memory_mb: {
                        let field_value = match fields_map.get("min_server_memory_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_server_memory_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
