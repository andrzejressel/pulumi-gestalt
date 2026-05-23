#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCloudExadataInfrastructuresCloudExadataInfrastructureProperty {
    /// The requested number of additional storage servers activated for the
    /// Exadata Infrastructure.
    #[builder(into)]
    pub r#activated_storage_count: i32,
    /// The requested number of additional storage servers for the Exadata
    /// Infrastructure.
    #[builder(into)]
    pub r#additional_storage_count: i32,
    /// The available storage can be allocated to the Exadata Infrastructure
    /// resource, in gigabytes (GB).
    #[builder(into)]
    pub r#available_storage_size_gb: i32,
    /// The number of compute servers for the Exadata Infrastructure.
    #[builder(into)]
    pub r#compute_count: i32,
    /// The number of enabled CPU cores.
    #[builder(into)]
    pub r#cpu_count: i32,
    /// The list of customer contacts.
    #[builder(into)]
    pub r#customer_contacts: Vec<super::super::types::oracledatabase::GetCloudExadataInfrastructuresCloudExadataInfrastructurePropertyCustomerContact>,
    /// Size, in terabytes, of the DATA disk group.
    #[builder(into)]
    pub r#data_storage_size_tb: f64,
    /// The local node storage allocated in GBs.
    #[builder(into)]
    pub r#db_node_storage_size_gb: i32,
    /// The software version of the database servers (dom0) in the Exadata
    /// Infrastructure.
    #[builder(into)]
    pub r#db_server_version: String,
    /// Maintenance window as defined by Oracle.
    /// https://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/MaintenanceWindow
    #[builder(into)]
    pub r#maintenance_windows: Vec<super::super::types::oracledatabase::GetCloudExadataInfrastructuresCloudExadataInfrastructurePropertyMaintenanceWindow>,
    /// The total number of CPU cores available.
    #[builder(into)]
    pub r#max_cpu_count: i32,
    /// The total available DATA disk group size.
    #[builder(into)]
    pub r#max_data_storage_tb: f64,
    /// The total local node storage available in GBs.
    #[builder(into)]
    pub r#max_db_node_storage_size_gb: i32,
    /// The total memory available in GBs.
    #[builder(into)]
    pub r#max_memory_gb: i32,
    /// The memory allocated in GBs.
    #[builder(into)]
    pub r#memory_size_gb: i32,
    /// The monthly software version of the database servers (dom0)
    /// in the Exadata Infrastructure. Example: 20.1.15
    #[builder(into)]
    pub r#monthly_db_server_version: String,
    /// The monthly software version of the storage servers (cells)
    /// in the Exadata Infrastructure. Example: 20.1.15
    #[builder(into)]
    pub r#monthly_storage_server_version: String,
    /// The OCID of the next maintenance run.
    #[builder(into)]
    pub r#next_maintenance_run_id: String,
    /// The time when the next maintenance run will occur.
    #[builder(into)]
    pub r#next_maintenance_run_time: String,
    /// The time when the next security maintenance run will occur.
    #[builder(into)]
    pub r#next_security_maintenance_run_time: String,
    /// Deep link to the OCI console to view this resource.
    #[builder(into)]
    pub r#oci_url: String,
    /// OCID of created infra.
    /// https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm#Oracle
    #[builder(into)]
    pub r#ocid: String,
    /// The shape of the Exadata Infrastructure. The shape determines the
    /// amount of CPU, storage, and memory resources allocated to the instance.
    #[builder(into)]
    pub r#shape: String,
    /// The current lifecycle state of the Exadata Infrastructure. 
    ///  Possible values:
    ///  STATE_UNSPECIFIED
    /// PROVISIONING
    /// AVAILABLE
    /// UPDATING
    /// TERMINATING
    /// TERMINATED
    /// FAILED
    /// MAINTENANCE_IN_PROGRESS
    #[builder(into)]
    pub r#state: String,
    /// The number of Cloud Exadata storage servers for the Exadata Infrastructure.
    #[builder(into)]
    pub r#storage_count: i32,
    /// The software version of the storage servers (cells) in the Exadata
    /// Infrastructure.
    #[builder(into)]
    pub r#storage_server_version: String,
    /// The total storage allocated to the Exadata Infrastructure
    /// resource, in gigabytes (GB).
    #[builder(into)]
    pub r#total_storage_size_gb: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCloudExadataInfrastructuresCloudExadataInfrastructureProperty {
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
                    "activatedStorageCount",
                    &self.r#activated_storage_count,
                ),
                to_pulumi_object_field(
                    "additionalStorageCount",
                    &self.r#additional_storage_count,
                ),
                to_pulumi_object_field(
                    "availableStorageSizeGb",
                    &self.r#available_storage_size_gb,
                ),
                to_pulumi_object_field(
                    "computeCount",
                    &self.r#compute_count,
                ),
                to_pulumi_object_field(
                    "cpuCount",
                    &self.r#cpu_count,
                ),
                to_pulumi_object_field(
                    "customerContacts",
                    &self.r#customer_contacts,
                ),
                to_pulumi_object_field(
                    "dataStorageSizeTb",
                    &self.r#data_storage_size_tb,
                ),
                to_pulumi_object_field(
                    "dbNodeStorageSizeGb",
                    &self.r#db_node_storage_size_gb,
                ),
                to_pulumi_object_field(
                    "dbServerVersion",
                    &self.r#db_server_version,
                ),
                to_pulumi_object_field(
                    "maintenanceWindows",
                    &self.r#maintenance_windows,
                ),
                to_pulumi_object_field(
                    "maxCpuCount",
                    &self.r#max_cpu_count,
                ),
                to_pulumi_object_field(
                    "maxDataStorageTb",
                    &self.r#max_data_storage_tb,
                ),
                to_pulumi_object_field(
                    "maxDbNodeStorageSizeGb",
                    &self.r#max_db_node_storage_size_gb,
                ),
                to_pulumi_object_field(
                    "maxMemoryGb",
                    &self.r#max_memory_gb,
                ),
                to_pulumi_object_field(
                    "memorySizeGb",
                    &self.r#memory_size_gb,
                ),
                to_pulumi_object_field(
                    "monthlyDbServerVersion",
                    &self.r#monthly_db_server_version,
                ),
                to_pulumi_object_field(
                    "monthlyStorageServerVersion",
                    &self.r#monthly_storage_server_version,
                ),
                to_pulumi_object_field(
                    "nextMaintenanceRunId",
                    &self.r#next_maintenance_run_id,
                ),
                to_pulumi_object_field(
                    "nextMaintenanceRunTime",
                    &self.r#next_maintenance_run_time,
                ),
                to_pulumi_object_field(
                    "nextSecurityMaintenanceRunTime",
                    &self.r#next_security_maintenance_run_time,
                ),
                to_pulumi_object_field(
                    "ociUrl",
                    &self.r#oci_url,
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
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "storageCount",
                    &self.r#storage_count,
                ),
                to_pulumi_object_field(
                    "storageServerVersion",
                    &self.r#storage_server_version,
                ),
                to_pulumi_object_field(
                    "totalStorageSizeGb",
                    &self.r#total_storage_size_gb,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCloudExadataInfrastructuresCloudExadataInfrastructureProperty {
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
                    r#activated_storage_count: {
                        let field_value = match fields_map.get("activatedStorageCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'activatedStorageCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#additional_storage_count: {
                        let field_value = match fields_map.get("additionalStorageCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'additionalStorageCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_storage_size_gb: {
                        let field_value = match fields_map.get("availableStorageSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'availableStorageSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compute_count: {
                        let field_value = match fields_map.get("computeCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'computeCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_count: {
                        let field_value = match fields_map.get("cpuCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpuCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customer_contacts: {
                        let field_value = match fields_map.get("customerContacts") {
                            Some(value) => value,
                            None => bail!("Missing field 'customerContacts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_storage_size_tb: {
                        let field_value = match fields_map.get("dataStorageSizeTb") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataStorageSizeTb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_node_storage_size_gb: {
                        let field_value = match fields_map.get("dbNodeStorageSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'dbNodeStorageSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_server_version: {
                        let field_value = match fields_map.get("dbServerVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'dbServerVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_windows: {
                        let field_value = match fields_map.get("maintenanceWindows") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenanceWindows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#max_data_storage_tb: {
                        let field_value = match fields_map.get("maxDataStorageTb") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxDataStorageTb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_db_node_storage_size_gb: {
                        let field_value = match fields_map.get("maxDbNodeStorageSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxDbNodeStorageSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_memory_gb: {
                        let field_value = match fields_map.get("maxMemoryGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxMemoryGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_size_gb: {
                        let field_value = match fields_map.get("memorySizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'memorySizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthly_db_server_version: {
                        let field_value = match fields_map.get("monthlyDbServerVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthlyDbServerVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthly_storage_server_version: {
                        let field_value = match fields_map.get("monthlyStorageServerVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthlyStorageServerVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_maintenance_run_id: {
                        let field_value = match fields_map.get("nextMaintenanceRunId") {
                            Some(value) => value,
                            None => bail!("Missing field 'nextMaintenanceRunId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_maintenance_run_time: {
                        let field_value = match fields_map.get("nextMaintenanceRunTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'nextMaintenanceRunTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_security_maintenance_run_time: {
                        let field_value = match fields_map.get("nextSecurityMaintenanceRunTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'nextSecurityMaintenanceRunTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oci_url: {
                        let field_value = match fields_map.get("ociUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'ociUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_count: {
                        let field_value = match fields_map.get("storageCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_server_version: {
                        let field_value = match fields_map.get("storageServerVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageServerVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_storage_size_gb: {
                        let field_value = match fields_map.get("totalStorageSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'totalStorageSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
