#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDbServersDbServer {
    /// The list of [OCIDs](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Autonomous Virtual Machines associated with the DB Server.
    #[builder(into)]
    #[serde(rename = "autonomousVirtualMachineDs")]
    pub r#autonomous_virtual_machine_ds: Vec<String>,
    /// The list of [OCIDs](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Autonomous VM Clusters associated with the DB Server.
    #[builder(into)]
    #[serde(rename = "autonomousVmClusterIds")]
    pub r#autonomous_vm_cluster_ids: Vec<String>,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the compartment.
    #[builder(into)]
    #[serde(rename = "compartmentId")]
    pub r#compartment_id: String,
    /// The number of CPU cores enabled on the DB Server.
    #[builder(into)]
    #[serde(rename = "cpuCoreCount")]
    pub r#cpu_core_count: i32,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Db nodes associated with the DB Server.
    #[builder(into)]
    #[serde(rename = "dbNodeIds")]
    pub r#db_node_ids: Vec<String>,
    /// The allocated local node storage in GBs on the DB Server.
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeInGbs")]
    pub r#db_node_storage_size_in_gbs: i32,
    /// The user-friendly name for the DB Server. The name does not need to be unique.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Exadata infrastructure.
    #[builder(into)]
    #[serde(rename = "exadataInfrastructureId")]
    pub r#exadata_infrastructure_id: String,
    /// Additional information about the current lifecycle state.
    #[builder(into)]
    #[serde(rename = "lifecycleDetails")]
    pub r#lifecycle_details: String,
    /// The current state of the DB Server.
    #[builder(into)]
    #[serde(rename = "lifecycleState")]
    pub r#lifecycle_state: String,
    /// The total number of CPU cores available.
    #[builder(into)]
    #[serde(rename = "maxCpuCount")]
    pub r#max_cpu_count: i32,
    /// The total local node storage available in GBs.
    #[builder(into)]
    #[serde(rename = "maxDbNodeStorageInGbs")]
    pub r#max_db_node_storage_in_gbs: i32,
    /// The total memory available in GBs.
    #[builder(into)]
    #[serde(rename = "maxMemoryInGbs")]
    pub r#max_memory_in_gbs: i32,
    /// The allocated memory in GBs on the DB Server.
    #[builder(into)]
    #[serde(rename = "memorySizeInGbs")]
    pub r#memory_size_in_gbs: i32,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the DB Server.
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: String,
    /// The shape of the DB Server. The shape determines the amount of CPU, storage, and memory resources available.
    #[builder(into)]
    #[serde(rename = "shape")]
    pub r#shape: String,
    /// The date and time that the DB Server was created.
    #[builder(into)]
    #[serde(rename = "timeCreated")]
    pub r#time_created: String,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the VM Clusters associated with the DB Server.
    #[builder(into)]
    #[serde(rename = "vmClusterIds")]
    pub r#vm_cluster_ids: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDbServersDbServer {
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
                "autonomous_virtual_machine_ds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#autonomous_virtual_machine_ds,
                )
                .await,
            );
            map.insert(
                "autonomous_vm_cluster_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#autonomous_vm_cluster_ids,
                )
                .await,
            );
            map.insert(
                "compartment_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compartment_id,
                )
                .await,
            );
            map.insert(
                "cpu_core_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu_core_count,
                )
                .await,
            );
            map.insert(
                "db_node_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#db_node_ids,
                )
                .await,
            );
            map.insert(
                "db_node_storage_size_in_gbs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#db_node_storage_size_in_gbs,
                )
                .await,
            );
            map.insert(
                "display_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#display_name,
                )
                .await,
            );
            map.insert(
                "exadata_infrastructure_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exadata_infrastructure_id,
                )
                .await,
            );
            map.insert(
                "lifecycle_details".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lifecycle_details,
                )
                .await,
            );
            map.insert(
                "lifecycle_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lifecycle_state,
                )
                .await,
            );
            map.insert(
                "max_cpu_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_cpu_count,
                )
                .await,
            );
            map.insert(
                "max_db_node_storage_in_gbs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_db_node_storage_in_gbs,
                )
                .await,
            );
            map.insert(
                "max_memory_in_gbs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_memory_in_gbs,
                )
                .await,
            );
            map.insert(
                "memory_size_in_gbs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#memory_size_in_gbs,
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
                "shape".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#shape,
                )
                .await,
            );
            map.insert(
                "time_created".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_created,
                )
                .await,
            );
            map.insert(
                "vm_cluster_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vm_cluster_ids,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDbServersDbServer {
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
                    r#autonomous_virtual_machine_ds: {
                        let field_value = match fields_map.get("autonomous_virtual_machine_ds") {
                            Some(value) => value,
                            None => bail!("Missing field 'autonomous_virtual_machine_ds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#autonomous_vm_cluster_ids: {
                        let field_value = match fields_map.get("autonomous_vm_cluster_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'autonomous_vm_cluster_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compartment_id: {
                        let field_value = match fields_map.get("compartment_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'compartment_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_core_count: {
                        let field_value = match fields_map.get("cpu_core_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_core_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_node_ids: {
                        let field_value = match fields_map.get("db_node_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_node_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_node_storage_size_in_gbs: {
                        let field_value = match fields_map.get("db_node_storage_size_in_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_node_storage_size_in_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exadata_infrastructure_id: {
                        let field_value = match fields_map.get("exadata_infrastructure_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'exadata_infrastructure_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifecycle_details: {
                        let field_value = match fields_map.get("lifecycle_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycle_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifecycle_state: {
                        let field_value = match fields_map.get("lifecycle_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycle_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_cpu_count: {
                        let field_value = match fields_map.get("max_cpu_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_cpu_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_db_node_storage_in_gbs: {
                        let field_value = match fields_map.get("max_db_node_storage_in_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_db_node_storage_in_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_memory_in_gbs: {
                        let field_value = match fields_map.get("max_memory_in_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_memory_in_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_size_in_gbs: {
                        let field_value = match fields_map.get("memory_size_in_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_size_in_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#shape: {
                        let field_value = match fields_map.get("shape") {
                            Some(value) => value,
                            None => bail!("Missing field 'shape' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_created: {
                        let field_value = match fields_map.get("time_created") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_created' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_cluster_ids: {
                        let field_value = match fields_map.get("vm_cluster_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_cluster_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
