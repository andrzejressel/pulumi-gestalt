#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCloudExadataInfrastructuresCloudExadataInfrastructureProperty {
    /// The requested number of additional storage servers activated for the
    /// Exadata Infrastructure.
    #[builder(into)]
    #[serde(rename = "activatedStorageCount")]
    pub r#activated_storage_count: i32,
    /// The requested number of additional storage servers for the Exadata
    /// Infrastructure.
    #[builder(into)]
    #[serde(rename = "additionalStorageCount")]
    pub r#additional_storage_count: i32,
    /// The available storage can be allocated to the Exadata Infrastructure
    /// resource, in gigabytes (GB).
    #[builder(into)]
    #[serde(rename = "availableStorageSizeGb")]
    pub r#available_storage_size_gb: i32,
    /// The number of compute servers for the Exadata Infrastructure.
    #[builder(into)]
    #[serde(rename = "computeCount")]
    pub r#compute_count: i32,
    /// The number of enabled CPU cores.
    #[builder(into)]
    #[serde(rename = "cpuCount")]
    pub r#cpu_count: i32,
    /// The list of customer contacts.
    #[builder(into)]
    #[serde(rename = "customerContacts")]
    pub r#customer_contacts: Vec<super::super::types::oracledatabase::GetCloudExadataInfrastructuresCloudExadataInfrastructurePropertyCustomerContact>,
    /// Size, in terabytes, of the DATA disk group.
    #[builder(into)]
    #[serde(rename = "dataStorageSizeTb")]
    pub r#data_storage_size_tb: f64,
    /// The local node storage allocated in GBs.
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeGb")]
    pub r#db_node_storage_size_gb: i32,
    /// The software version of the database servers (dom0) in the Exadata
    /// Infrastructure.
    #[builder(into)]
    #[serde(rename = "dbServerVersion")]
    pub r#db_server_version: String,
    /// Maintenance window as defined by Oracle.
    /// https://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/MaintenanceWindow
    #[builder(into)]
    #[serde(rename = "maintenanceWindows")]
    pub r#maintenance_windows: Vec<super::super::types::oracledatabase::GetCloudExadataInfrastructuresCloudExadataInfrastructurePropertyMaintenanceWindow>,
    /// The total number of CPU cores available.
    #[builder(into)]
    #[serde(rename = "maxCpuCount")]
    pub r#max_cpu_count: i32,
    /// The total available DATA disk group size.
    #[builder(into)]
    #[serde(rename = "maxDataStorageTb")]
    pub r#max_data_storage_tb: f64,
    /// The total local node storage available in GBs.
    #[builder(into)]
    #[serde(rename = "maxDbNodeStorageSizeGb")]
    pub r#max_db_node_storage_size_gb: i32,
    /// The total memory available in GBs.
    #[builder(into)]
    #[serde(rename = "maxMemoryGb")]
    pub r#max_memory_gb: i32,
    /// The memory allocated in GBs.
    #[builder(into)]
    #[serde(rename = "memorySizeGb")]
    pub r#memory_size_gb: i32,
    /// The monthly software version of the database servers (dom0)
    /// in the Exadata Infrastructure. Example: 20.1.15
    #[builder(into)]
    #[serde(rename = "monthlyDbServerVersion")]
    pub r#monthly_db_server_version: String,
    /// The monthly software version of the storage servers (cells)
    /// in the Exadata Infrastructure. Example: 20.1.15
    #[builder(into)]
    #[serde(rename = "monthlyStorageServerVersion")]
    pub r#monthly_storage_server_version: String,
    /// The OCID of the next maintenance run.
    #[builder(into)]
    #[serde(rename = "nextMaintenanceRunId")]
    pub r#next_maintenance_run_id: String,
    /// The time when the next maintenance run will occur.
    #[builder(into)]
    #[serde(rename = "nextMaintenanceRunTime")]
    pub r#next_maintenance_run_time: String,
    /// The time when the next security maintenance run will occur.
    #[builder(into)]
    #[serde(rename = "nextSecurityMaintenanceRunTime")]
    pub r#next_security_maintenance_run_time: String,
    /// Deep link to the OCI console to view this resource.
    #[builder(into)]
    #[serde(rename = "ociUrl")]
    pub r#oci_url: String,
    /// OCID of created infra.
    /// https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm#Oracle
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: String,
    /// The shape of the Exadata Infrastructure. The shape determines the
    /// amount of CPU, storage, and memory resources allocated to the instance.
    #[builder(into)]
    #[serde(rename = "shape")]
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
    #[serde(rename = "state")]
    pub r#state: String,
    /// The number of Cloud Exadata storage servers for the Exadata Infrastructure.
    #[builder(into)]
    #[serde(rename = "storageCount")]
    pub r#storage_count: i32,
    /// The software version of the storage servers (cells) in the Exadata
    /// Infrastructure.
    #[builder(into)]
    #[serde(rename = "storageServerVersion")]
    pub r#storage_server_version: String,
    /// The total storage allocated to the Exadata Infrastructure
    /// resource, in gigabytes (GB).
    #[builder(into)]
    #[serde(rename = "totalStorageSizeGb")]
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
                    "activated_storage_count",
                    &self.r#activated_storage_count,
                ),
                to_pulumi_object_field(
                    "additional_storage_count",
                    &self.r#additional_storage_count,
                ),
                to_pulumi_object_field(
                    "available_storage_size_gb",
                    &self.r#available_storage_size_gb,
                ),
                to_pulumi_object_field(
                    "compute_count",
                    &self.r#compute_count,
                ),
                to_pulumi_object_field(
                    "cpu_count",
                    &self.r#cpu_count,
                ),
                to_pulumi_object_field(
                    "customer_contacts",
                    &self.r#customer_contacts,
                ),
                to_pulumi_object_field(
                    "data_storage_size_tb",
                    &self.r#data_storage_size_tb,
                ),
                to_pulumi_object_field(
                    "db_node_storage_size_gb",
                    &self.r#db_node_storage_size_gb,
                ),
                to_pulumi_object_field(
                    "db_server_version",
                    &self.r#db_server_version,
                ),
                to_pulumi_object_field(
                    "maintenance_windows",
                    &self.r#maintenance_windows,
                ),
                to_pulumi_object_field(
                    "max_cpu_count",
                    &self.r#max_cpu_count,
                ),
                to_pulumi_object_field(
                    "max_data_storage_tb",
                    &self.r#max_data_storage_tb,
                ),
                to_pulumi_object_field(
                    "max_db_node_storage_size_gb",
                    &self.r#max_db_node_storage_size_gb,
                ),
                to_pulumi_object_field(
                    "max_memory_gb",
                    &self.r#max_memory_gb,
                ),
                to_pulumi_object_field(
                    "memory_size_gb",
                    &self.r#memory_size_gb,
                ),
                to_pulumi_object_field(
                    "monthly_db_server_version",
                    &self.r#monthly_db_server_version,
                ),
                to_pulumi_object_field(
                    "monthly_storage_server_version",
                    &self.r#monthly_storage_server_version,
                ),
                to_pulumi_object_field(
                    "next_maintenance_run_id",
                    &self.r#next_maintenance_run_id,
                ),
                to_pulumi_object_field(
                    "next_maintenance_run_time",
                    &self.r#next_maintenance_run_time,
                ),
                to_pulumi_object_field(
                    "next_security_maintenance_run_time",
                    &self.r#next_security_maintenance_run_time,
                ),
                to_pulumi_object_field(
                    "oci_url",
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
                    "storage_count",
                    &self.r#storage_count,
                ),
                to_pulumi_object_field(
                    "storage_server_version",
                    &self.r#storage_server_version,
                ),
                to_pulumi_object_field(
                    "total_storage_size_gb",
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
                        let field_value = match fields_map.get("activated_storage_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'activated_storage_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#additional_storage_count: {
                        let field_value = match fields_map.get("additional_storage_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_storage_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_storage_size_gb: {
                        let field_value = match fields_map.get("available_storage_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_storage_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compute_count: {
                        let field_value = match fields_map.get("compute_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'compute_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_count: {
                        let field_value = match fields_map.get("cpu_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customer_contacts: {
                        let field_value = match fields_map.get("customer_contacts") {
                            Some(value) => value,
                            None => bail!("Missing field 'customer_contacts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_storage_size_tb: {
                        let field_value = match fields_map.get("data_storage_size_tb") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_storage_size_tb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#db_server_version: {
                        let field_value = match fields_map.get("db_server_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_server_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_windows: {
                        let field_value = match fields_map.get("maintenance_windows") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_windows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#max_data_storage_tb: {
                        let field_value = match fields_map.get("max_data_storage_tb") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_data_storage_tb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#max_memory_gb: {
                        let field_value = match fields_map.get("max_memory_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_memory_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#monthly_db_server_version: {
                        let field_value = match fields_map.get("monthly_db_server_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthly_db_server_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthly_storage_server_version: {
                        let field_value = match fields_map.get("monthly_storage_server_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthly_storage_server_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_maintenance_run_id: {
                        let field_value = match fields_map.get("next_maintenance_run_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_maintenance_run_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_maintenance_run_time: {
                        let field_value = match fields_map.get("next_maintenance_run_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_maintenance_run_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_security_maintenance_run_time: {
                        let field_value = match fields_map.get("next_security_maintenance_run_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_security_maintenance_run_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oci_url: {
                        let field_value = match fields_map.get("oci_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'oci_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("storage_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_server_version: {
                        let field_value = match fields_map.get("storage_server_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_server_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_storage_size_gb: {
                        let field_value = match fields_map.get("total_storage_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'total_storage_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
