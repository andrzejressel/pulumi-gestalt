#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDbNodesDbNode {
    /// Additional information about the planned maintenance.
    #[builder(into)]
    pub r#additional_details: String,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the backup IP address associated with the database node. Use this OCID with either the [GetPrivateIp](https://docs.cloud.oracle.com/iaas/api/#/en/iaas/20160918/PrivateIp/GetPrivateIp) or the [GetPublicIpByPrivateIpId](https://docs.cloud.oracle.com/iaas/api/#/en/iaas/20160918/PublicIp/GetPublicIpByPrivateIpId) API to get the IP address needed to make a database connection.
    #[builder(into)]
    pub r#backup_ip_id: String,
    #[builder(into)]
    pub r#backup_vnic_2_id: String,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the backup VNIC.
    #[builder(into)]
    pub r#backup_vnic_id: String,
    /// The number of CPU cores enabled on the DB node.
    #[builder(into)]
    pub r#cpu_core_count: i32,
    /// The allocated local node storage in GBs on the DB node.
    #[builder(into)]
    pub r#db_node_storage_size_in_gbs: i32,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the ExaCC DB server associated with the database node.
    #[builder(into)]
    pub r#db_server_id: String,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the DB system.
    #[builder(into)]
    pub r#db_system_id: String,
    /// The name of the Fault Domain the instance is contained in.
    #[builder(into)]
    pub r#fault_domain: String,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the host IP address associated with the database node. Use this OCID with either the [GetPrivateIp](https://docs.cloud.oracle.com/iaas/api/#/en/iaas/20160918/PrivateIp/GetPrivateIp) or the [GetPublicIpByPrivateIpId](https://docs.cloud.oracle.com/iaas/api/#/en/iaas/20160918/PublicIp/GetPublicIpByPrivateIpId) API to get the IP address needed to make a database connection.
    #[builder(into)]
    pub r#host_ip_id: String,
    #[builder(into)]
    pub r#hostname: String,
    /// Information about the current lifecycle details.
    #[builder(into)]
    pub r#lifecycle_details: String,
    /// Information about the current lifecycle state.
    #[builder(into)]
    pub r#lifecycle_state: String,
    /// The type of database node maintenance.
    #[builder(into)]
    pub r#maintenance_type: String,
    /// The allocated memory in GBs on the DB Node.
    #[builder(into)]
    pub r#memory_size_in_gbs: i32,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the DB node.
    #[builder(into)]
    pub r#ocid: String,
    /// The size (in GB) of the block storage volume allocation for the DB system. This attribute applies only for virtual machine DB systems.
    #[builder(into)]
    pub r#software_storage_size_in_gb: i32,
    /// The date and time that the DB node was created.
    #[builder(into)]
    pub r#time_created: String,
    /// End date and time of maintenance window.
    #[builder(into)]
    pub r#time_maintenance_window_end: String,
    /// Start date and time of maintenance window.
    #[builder(into)]
    pub r#time_maintenance_window_start: String,
    #[builder(into)]
    pub r#vnic_2_id: String,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the VNIC.
    #[builder(into)]
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
                    "additionalDetails",
                    &self.r#additional_details,
                ),
                to_pulumi_object_field(
                    "backupIpId",
                    &self.r#backup_ip_id,
                ),
                to_pulumi_object_field(
                    "backupVnic2Id",
                    &self.r#backup_vnic_2_id,
                ),
                to_pulumi_object_field(
                    "backupVnicId",
                    &self.r#backup_vnic_id,
                ),
                to_pulumi_object_field(
                    "cpuCoreCount",
                    &self.r#cpu_core_count,
                ),
                to_pulumi_object_field(
                    "dbNodeStorageSizeInGbs",
                    &self.r#db_node_storage_size_in_gbs,
                ),
                to_pulumi_object_field(
                    "dbServerId",
                    &self.r#db_server_id,
                ),
                to_pulumi_object_field(
                    "dbSystemId",
                    &self.r#db_system_id,
                ),
                to_pulumi_object_field(
                    "faultDomain",
                    &self.r#fault_domain,
                ),
                to_pulumi_object_field(
                    "hostIpId",
                    &self.r#host_ip_id,
                ),
                to_pulumi_object_field(
                    "hostname",
                    &self.r#hostname,
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
                    "maintenanceType",
                    &self.r#maintenance_type,
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
                    "softwareStorageSizeInGb",
                    &self.r#software_storage_size_in_gb,
                ),
                to_pulumi_object_field(
                    "timeCreated",
                    &self.r#time_created,
                ),
                to_pulumi_object_field(
                    "timeMaintenanceWindowEnd",
                    &self.r#time_maintenance_window_end,
                ),
                to_pulumi_object_field(
                    "timeMaintenanceWindowStart",
                    &self.r#time_maintenance_window_start,
                ),
                to_pulumi_object_field(
                    "vnic2Id",
                    &self.r#vnic_2_id,
                ),
                to_pulumi_object_field(
                    "vnicId",
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
                        let field_value = match fields_map.get("additionalDetails") {
                            Some(value) => value,
                            None => bail!("Missing field 'additionalDetails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_ip_id: {
                        let field_value = match fields_map.get("backupIpId") {
                            Some(value) => value,
                            None => bail!("Missing field 'backupIpId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_vnic_2_id: {
                        let field_value = match fields_map.get("backupVnic2Id") {
                            Some(value) => value,
                            None => bail!("Missing field 'backupVnic2Id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_vnic_id: {
                        let field_value = match fields_map.get("backupVnicId") {
                            Some(value) => value,
                            None => bail!("Missing field 'backupVnicId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#db_node_storage_size_in_gbs: {
                        let field_value = match fields_map.get("dbNodeStorageSizeInGbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'dbNodeStorageSizeInGbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_server_id: {
                        let field_value = match fields_map.get("dbServerId") {
                            Some(value) => value,
                            None => bail!("Missing field 'dbServerId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_system_id: {
                        let field_value = match fields_map.get("dbSystemId") {
                            Some(value) => value,
                            None => bail!("Missing field 'dbSystemId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fault_domain: {
                        let field_value = match fields_map.get("faultDomain") {
                            Some(value) => value,
                            None => bail!("Missing field 'faultDomain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_ip_id: {
                        let field_value = match fields_map.get("hostIpId") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostIpId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#maintenance_type: {
                        let field_value = match fields_map.get("maintenanceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenanceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#software_storage_size_in_gb: {
                        let field_value = match fields_map.get("softwareStorageSizeInGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'softwareStorageSizeInGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#time_maintenance_window_end: {
                        let field_value = match fields_map.get("timeMaintenanceWindowEnd") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeMaintenanceWindowEnd' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_maintenance_window_start: {
                        let field_value = match fields_map.get("timeMaintenanceWindowStart") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeMaintenanceWindowStart' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnic_2_id: {
                        let field_value = match fields_map.get("vnic2Id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnic2Id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnic_id: {
                        let field_value = match fields_map.get("vnicId") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnicId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
