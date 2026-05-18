#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDbNodesDbNode {
    /// Additional information about the planned maintenance.
    #[builder(into)]
    #[serde(rename = "additionalDetails")]
    pub r#additional_details: String,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the backup IP address associated with the database node. Use this OCID with either the [GetPrivateIp](https://docs.cloud.oracle.com/iaas/api/#/en/iaas/20160918/PrivateIp/GetPrivateIp) or the [GetPublicIpByPrivateIpId](https://docs.cloud.oracle.com/iaas/api/#/en/iaas/20160918/PublicIp/GetPublicIpByPrivateIpId) API to get the IP address needed to make a database connection.
    #[builder(into)]
    #[serde(rename = "backupIpId")]
    pub r#backup_ip_id: String,
    #[builder(into)]
    #[serde(rename = "backupVnic2Id")]
    pub r#backup_vnic_2_id: String,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the backup VNIC.
    #[builder(into)]
    #[serde(rename = "backupVnicId")]
    pub r#backup_vnic_id: String,
    /// The number of CPU cores enabled on the DB node.
    #[builder(into)]
    #[serde(rename = "cpuCoreCount")]
    pub r#cpu_core_count: i32,
    /// The allocated local node storage in GBs on the DB node.
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeInGbs")]
    pub r#db_node_storage_size_in_gbs: i32,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the ExaCC DB server associated with the database node.
    #[builder(into)]
    #[serde(rename = "dbServerId")]
    pub r#db_server_id: String,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the DB system.
    #[builder(into)]
    #[serde(rename = "dbSystemId")]
    pub r#db_system_id: String,
    /// The name of the Fault Domain the instance is contained in.
    #[builder(into)]
    #[serde(rename = "faultDomain")]
    pub r#fault_domain: String,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the host IP address associated with the database node. Use this OCID with either the [GetPrivateIp](https://docs.cloud.oracle.com/iaas/api/#/en/iaas/20160918/PrivateIp/GetPrivateIp) or the [GetPublicIpByPrivateIpId](https://docs.cloud.oracle.com/iaas/api/#/en/iaas/20160918/PublicIp/GetPublicIpByPrivateIpId) API to get the IP address needed to make a database connection.
    #[builder(into)]
    #[serde(rename = "hostIpId")]
    pub r#host_ip_id: String,
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: String,
    /// Information about the current lifecycle details.
    #[builder(into)]
    #[serde(rename = "lifecycleDetails")]
    pub r#lifecycle_details: String,
    /// Information about the current lifecycle state.
    #[builder(into)]
    #[serde(rename = "lifecycleState")]
    pub r#lifecycle_state: String,
    /// The type of database node maintenance.
    #[builder(into)]
    #[serde(rename = "maintenanceType")]
    pub r#maintenance_type: String,
    /// The allocated memory in GBs on the DB Node.
    #[builder(into)]
    #[serde(rename = "memorySizeInGbs")]
    pub r#memory_size_in_gbs: i32,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the DB node.
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: String,
    /// The size (in GB) of the block storage volume allocation for the DB system. This attribute applies only for virtual machine DB systems.
    #[builder(into)]
    #[serde(rename = "softwareStorageSizeInGb")]
    pub r#software_storage_size_in_gb: i32,
    /// The date and time that the DB node was created.
    #[builder(into)]
    #[serde(rename = "timeCreated")]
    pub r#time_created: String,
    /// End date and time of maintenance window.
    #[builder(into)]
    #[serde(rename = "timeMaintenanceWindowEnd")]
    pub r#time_maintenance_window_end: String,
    /// Start date and time of maintenance window.
    #[builder(into)]
    #[serde(rename = "timeMaintenanceWindowStart")]
    pub r#time_maintenance_window_start: String,
    #[builder(into)]
    #[serde(rename = "vnic2Id")]
    pub r#vnic_2_id: String,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the VNIC.
    #[builder(into)]
    #[serde(rename = "vnicId")]
    pub r#vnic_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDbNodesDbNode {
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
                    "additional_details",
                    &self.r#additional_details,
                ),
                to_pulumi_object_field(
                    "backup_ip_id",
                    &self.r#backup_ip_id,
                ),
                to_pulumi_object_field(
                    "backup_vnic_2_id",
                    &self.r#backup_vnic_2_id,
                ),
                to_pulumi_object_field(
                    "backup_vnic_id",
                    &self.r#backup_vnic_id,
                ),
                to_pulumi_object_field(
                    "cpu_core_count",
                    &self.r#cpu_core_count,
                ),
                to_pulumi_object_field(
                    "db_node_storage_size_in_gbs",
                    &self.r#db_node_storage_size_in_gbs,
                ),
                to_pulumi_object_field(
                    "db_server_id",
                    &self.r#db_server_id,
                ),
                to_pulumi_object_field(
                    "db_system_id",
                    &self.r#db_system_id,
                ),
                to_pulumi_object_field(
                    "fault_domain",
                    &self.r#fault_domain,
                ),
                to_pulumi_object_field(
                    "host_ip_id",
                    &self.r#host_ip_id,
                ),
                to_pulumi_object_field(
                    "hostname",
                    &self.r#hostname,
                ),
                to_pulumi_object_field(
                    "lifecycle_details",
                    &self.r#lifecycle_details,
                ),
                to_pulumi_object_field(
                    "lifecycle_state",
                    &self.r#lifecycle_state,
                ),
                to_pulumi_object_field(
                    "maintenance_type",
                    &self.r#maintenance_type,
                ),
                to_pulumi_object_field(
                    "memory_size_in_gbs",
                    &self.r#memory_size_in_gbs,
                ),
                to_pulumi_object_field(
                    "ocid",
                    &self.r#ocid,
                ),
                to_pulumi_object_field(
                    "software_storage_size_in_gb",
                    &self.r#software_storage_size_in_gb,
                ),
                to_pulumi_object_field(
                    "time_created",
                    &self.r#time_created,
                ),
                to_pulumi_object_field(
                    "time_maintenance_window_end",
                    &self.r#time_maintenance_window_end,
                ),
                to_pulumi_object_field(
                    "time_maintenance_window_start",
                    &self.r#time_maintenance_window_start,
                ),
                to_pulumi_object_field(
                    "vnic_2_id",
                    &self.r#vnic_2_id,
                ),
                to_pulumi_object_field(
                    "vnic_id",
                    &self.r#vnic_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDbNodesDbNode {
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
                    r#additional_details: {
                        let field_value = match fields_map.get("additional_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_ip_id: {
                        let field_value = match fields_map.get("backup_ip_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_ip_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_vnic_2_id: {
                        let field_value = match fields_map.get("backup_vnic_2_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_vnic_2_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_vnic_id: {
                        let field_value = match fields_map.get("backup_vnic_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_vnic_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#db_node_storage_size_in_gbs: {
                        let field_value = match fields_map.get("db_node_storage_size_in_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_node_storage_size_in_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_server_id: {
                        let field_value = match fields_map.get("db_server_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_server_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_system_id: {
                        let field_value = match fields_map.get("db_system_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_system_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fault_domain: {
                        let field_value = match fields_map.get("fault_domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'fault_domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_ip_id: {
                        let field_value = match fields_map.get("host_ip_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_ip_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hostname: {
                        let field_value = match fields_map.get("hostname") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#maintenance_type: {
                        let field_value = match fields_map.get("maintenance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#software_storage_size_in_gb: {
                        let field_value = match fields_map.get("software_storage_size_in_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'software_storage_size_in_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#time_maintenance_window_end: {
                        let field_value = match fields_map.get("time_maintenance_window_end") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_maintenance_window_end' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_maintenance_window_start: {
                        let field_value = match fields_map.get("time_maintenance_window_start") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_maintenance_window_start' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnic_2_id: {
                        let field_value = match fields_map.get("vnic_2_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnic_2_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnic_id: {
                        let field_value = match fields_map.get("vnic_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnic_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
