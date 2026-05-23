#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDbServersDbServer {
    /// The list of [OCIDs](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Autonomous Virtual Machines associated with the DB Server.
    #[builder(into)]
    pub r#autonomous_virtual_machine_ds: Vec<String>,
    /// The list of [OCIDs](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Autonomous VM Clusters associated with the DB Server.
    #[builder(into)]
    pub r#autonomous_vm_cluster_ids: Vec<String>,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the compartment.
    #[builder(into)]
    pub r#compartment_id: String,
    /// The number of CPU cores enabled on the DB Server.
    #[builder(into)]
    pub r#cpu_core_count: i32,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Db nodes associated with the DB Server.
    #[builder(into)]
    pub r#db_node_ids: Vec<String>,
    /// The allocated local node storage in GBs on the DB Server.
    #[builder(into)]
    pub r#db_node_storage_size_in_gbs: i32,
    /// The user-friendly name for the DB Server. The name does not need to be unique.
    #[builder(into)]
    pub r#display_name: String,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Exadata infrastructure.
    #[builder(into)]
    pub r#exadata_infrastructure_id: String,
    /// Additional information about the current lifecycle state.
    #[builder(into)]
    pub r#lifecycle_details: String,
    /// The current state of the DB Server.
    #[builder(into)]
    pub r#lifecycle_state: String,
    /// The total number of CPU cores available.
    #[builder(into)]
    pub r#max_cpu_count: i32,
    /// The total local node storage available in GBs.
    #[builder(into)]
    pub r#max_db_node_storage_in_gbs: i32,
    /// The total memory available in GBs.
    #[builder(into)]
    pub r#max_memory_in_gbs: i32,
    /// The allocated memory in GBs on the DB Server.
    #[builder(into)]
    pub r#memory_size_in_gbs: i32,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the DB Server.
    #[builder(into)]
    pub r#ocid: String,
    /// The shape of the DB Server. The shape determines the amount of CPU, storage, and memory resources available.
    #[builder(into)]
    pub r#shape: String,
    /// The date and time that the DB Server was created.
    #[builder(into)]
    pub r#time_created: String,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the VM Clusters associated with the DB Server.
    #[builder(into)]
    pub r#vm_cluster_ids: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDbServersDbServer {
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
                    "autonomousVirtualMachineDs",
                    &self.r#autonomous_virtual_machine_ds,
                ),
                to_pulumi_object_field(
                    "autonomousVmClusterIds",
                    &self.r#autonomous_vm_cluster_ids,
                ),
                to_pulumi_object_field(
                    "compartmentId",
                    &self.r#compartment_id,
                ),
                to_pulumi_object_field(
                    "cpuCoreCount",
                    &self.r#cpu_core_count,
                ),
                to_pulumi_object_field(
                    "dbNodeIds",
                    &self.r#db_node_ids,
                ),
                to_pulumi_object_field(
                    "dbNodeStorageSizeInGbs",
                    &self.r#db_node_storage_size_in_gbs,
                ),
                to_pulumi_object_field(
                    "displayName",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "exadataInfrastructureId",
                    &self.r#exadata_infrastructure_id,
                ),
                to_pulumi_object_field(
                    "lifecycleDetails",
                    &self.r#lifecycle_details,
                ),
                to_pulumi_object_field(
                    "lifecycleState",
                    &self.r#lifecycle_state,
                ),
                to_pulumi_object_field(
                    "maxCpuCount",
                    &self.r#max_cpu_count,
                ),
                to_pulumi_object_field(
                    "maxDbNodeStorageInGbs",
                    &self.r#max_db_node_storage_in_gbs,
                ),
                to_pulumi_object_field(
                    "maxMemoryInGbs",
                    &self.r#max_memory_in_gbs,
                ),
                to_pulumi_object_field(
                    "memorySizeInGbs",
                    &self.r#memory_size_in_gbs,
                ),
                to_pulumi_object_field(
                    "ocid",
                    &self.r#ocid,
                ),
                to_pulumi_object_field(
                    "shape",
                    &self.r#shape,
                ),
                to_pulumi_object_field(
                    "timeCreated",
                    &self.r#time_created,
                ),
                to_pulumi_object_field(
                    "vmClusterIds",
                    &self.r#vm_cluster_ids,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("autonomousVirtualMachineDs") {
                            Some(value) => value,
                            None => bail!("Missing field 'autonomousVirtualMachineDs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#autonomous_vm_cluster_ids: {
                        let field_value = match fields_map.get("autonomousVmClusterIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'autonomousVmClusterIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compartment_id: {
                        let field_value = match fields_map.get("compartmentId") {
                            Some(value) => value,
                            None => bail!("Missing field 'compartmentId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_core_count: {
                        let field_value = match fields_map.get("cpuCoreCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpuCoreCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_node_ids: {
                        let field_value = match fields_map.get("dbNodeIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'dbNodeIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_node_storage_size_in_gbs: {
                        let field_value = match fields_map.get("dbNodeStorageSizeInGbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'dbNodeStorageSizeInGbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("displayName") {
                            Some(value) => value,
                            None => bail!("Missing field 'displayName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exadata_infrastructure_id: {
                        let field_value = match fields_map.get("exadataInfrastructureId") {
                            Some(value) => value,
                            None => bail!("Missing field 'exadataInfrastructureId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifecycle_details: {
                        let field_value = match fields_map.get("lifecycleDetails") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycleDetails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifecycle_state: {
                        let field_value = match fields_map.get("lifecycleState") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycleState' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_cpu_count: {
                        let field_value = match fields_map.get("maxCpuCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxCpuCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_db_node_storage_in_gbs: {
                        let field_value = match fields_map.get("maxDbNodeStorageInGbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxDbNodeStorageInGbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_memory_in_gbs: {
                        let field_value = match fields_map.get("maxMemoryInGbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxMemoryInGbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_size_in_gbs: {
                        let field_value = match fields_map.get("memorySizeInGbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'memorySizeInGbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("timeCreated") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeCreated' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_cluster_ids: {
                        let field_value = match fields_map.get("vmClusterIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmClusterIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
