#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCloudVmClustersCloudVmClusterProperty {
    /// OCI Cluster name.
    #[builder(into)]
    #[serde(rename = "clusterName")]
    pub r#cluster_name: String,
    /// Compartment ID of cluster.
    #[builder(into)]
    #[serde(rename = "compartmentId")]
    pub r#compartment_id: String,
    /// Number of enabled CPU cores.
    #[builder(into)]
    #[serde(rename = "cpuCoreCount")]
    pub r#cpu_core_count: i32,
    /// The data disk group size to be allocated in TBs.
    #[builder(into)]
    #[serde(rename = "dataStorageSizeTb")]
    pub r#data_storage_size_tb: f64,
    /// Local storage per VM
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeGb")]
    pub r#db_node_storage_size_gb: i32,
    /// OCID of database servers.
    #[builder(into)]
    #[serde(rename = "dbServerOcids")]
    pub r#db_server_ocids: Vec<String>,
    /// Data collection options for diagnostics.
    #[builder(into)]
    #[serde(rename = "diagnosticsDataCollectionOptions")]
    pub r#diagnostics_data_collection_options: Vec<super::super::types::oracledatabase::GetCloudVmClustersCloudVmClusterPropertyDiagnosticsDataCollectionOption>,
    /// The type of redundancy. 
    ///  Possible values:
    ///  DISK_REDUNDANCY_UNSPECIFIED
    /// HIGH
    /// NORMAL
    #[builder(into)]
    #[serde(rename = "diskRedundancy")]
    pub r#disk_redundancy: String,
    /// DNS listener IP.
    #[builder(into)]
    #[serde(rename = "dnsListenerIp")]
    pub r#dns_listener_ip: String,
    /// Parent DNS domain where SCAN DNS and hosts names are qualified.
    /// ex: ocispdelegated.ocisp10jvnet.oraclevcn.com
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: String,
    /// Grid Infrastructure Version.
    #[builder(into)]
    #[serde(rename = "giVersion")]
    pub r#gi_version: String,
    /// host name without domain.
    /// format: "-" with some suffix.
    /// ex: sp2-yi0xq where "sp2" is the hostname_prefix.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: String,
    /// Prefix for VM cluster host names.
    #[builder(into)]
    #[serde(rename = "hostnamePrefix")]
    pub r#hostname_prefix: String,
    /// License type of VM Cluster. 
    ///  Possible values:
    ///  LICENSE_TYPE_UNSPECIFIED
    /// LICENSE_INCLUDED
    /// BRING_YOUR_OWN_LICENSE
    #[builder(into)]
    #[serde(rename = "licenseType")]
    pub r#license_type: String,
    /// Use local backup.
    #[builder(into)]
    #[serde(rename = "localBackupEnabled")]
    pub r#local_backup_enabled: bool,
    /// Memory allocated in GBs.
    #[builder(into)]
    #[serde(rename = "memorySizeGb")]
    pub r#memory_size_gb: i32,
    /// Number of database servers.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: i32,
    /// Deep link to the OCI console to view this resource.
    #[builder(into)]
    #[serde(rename = "ociUrl")]
    pub r#oci_url: String,
    /// Oracle Cloud Infrastructure ID of VM Cluster.
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: String,
    /// OCPU count per VM. Minimum is 0.1.
    #[builder(into)]
    #[serde(rename = "ocpuCount")]
    pub r#ocpu_count: f64,
    /// SCAN DNS name.
    /// ex: sp2-yi0xq-scan.ocispdelegated.ocisp10jvnet.oraclevcn.com
    #[builder(into)]
    #[serde(rename = "scanDns")]
    pub r#scan_dns: String,
    /// OCID of scan DNS record.
    #[builder(into)]
    #[serde(rename = "scanDnsRecordId")]
    pub r#scan_dns_record_id: String,
    /// OCIDs of scan IPs.
    #[builder(into)]
    #[serde(rename = "scanIpIds")]
    pub r#scan_ip_ids: Vec<String>,
    /// SCAN listener port - TCP
    #[builder(into)]
    #[serde(rename = "scanListenerPortTcp")]
    pub r#scan_listener_port_tcp: i32,
    /// SCAN listener port - TLS
    #[builder(into)]
    #[serde(rename = "scanListenerPortTcpSsl")]
    pub r#scan_listener_port_tcp_ssl: i32,
    /// Shape of VM Cluster.
    #[builder(into)]
    #[serde(rename = "shape")]
    pub r#shape: String,
    /// Use exadata sparse snapshots.
    #[builder(into)]
    #[serde(rename = "sparseDiskgroupEnabled")]
    pub r#sparse_diskgroup_enabled: bool,
    /// SSH public keys to be stored with cluster.
    #[builder(into)]
    #[serde(rename = "sshPublicKeys")]
    pub r#ssh_public_keys: Vec<String>,
    /// State of the cluster. 
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
    /// The storage allocation for the disk group, in gigabytes (GB).
    #[builder(into)]
    #[serde(rename = "storageSizeGb")]
    pub r#storage_size_gb: i32,
    /// Operating system version of the image.
    #[builder(into)]
    #[serde(rename = "systemVersion")]
    pub r#system_version: String,
    /// Represents a time zone from the
    /// [IANA Time Zone Database](https://www.iana.org/time-zones).
    #[builder(into)]
    #[serde(rename = "timeZones")]
    pub r#time_zones: Vec<super::super::types::oracledatabase::GetCloudVmClustersCloudVmClusterPropertyTimeZone>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCloudVmClustersCloudVmClusterProperty {
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
                    "cluster_name",
                    &self.r#cluster_name,
                ),
                to_pulumi_object_field(
                    "compartment_id",
                    &self.r#compartment_id,
                ),
                to_pulumi_object_field(
                    "cpu_core_count",
                    &self.r#cpu_core_count,
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
                    "db_server_ocids",
                    &self.r#db_server_ocids,
                ),
                to_pulumi_object_field(
                    "diagnostics_data_collection_options",
                    &self.r#diagnostics_data_collection_options,
                ),
                to_pulumi_object_field(
                    "disk_redundancy",
                    &self.r#disk_redundancy,
                ),
                to_pulumi_object_field(
                    "dns_listener_ip",
                    &self.r#dns_listener_ip,
                ),
                to_pulumi_object_field(
                    "domain",
                    &self.r#domain,
                ),
                to_pulumi_object_field(
                    "gi_version",
                    &self.r#gi_version,
                ),
                to_pulumi_object_field(
                    "hostname",
                    &self.r#hostname,
                ),
                to_pulumi_object_field(
                    "hostname_prefix",
                    &self.r#hostname_prefix,
                ),
                to_pulumi_object_field(
                    "license_type",
                    &self.r#license_type,
                ),
                to_pulumi_object_field(
                    "local_backup_enabled",
                    &self.r#local_backup_enabled,
                ),
                to_pulumi_object_field(
                    "memory_size_gb",
                    &self.r#memory_size_gb,
                ),
                to_pulumi_object_field(
                    "node_count",
                    &self.r#node_count,
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
                    "ocpu_count",
                    &self.r#ocpu_count,
                ),
                to_pulumi_object_field(
                    "scan_dns",
                    &self.r#scan_dns,
                ),
                to_pulumi_object_field(
                    "scan_dns_record_id",
                    &self.r#scan_dns_record_id,
                ),
                to_pulumi_object_field(
                    "scan_ip_ids",
                    &self.r#scan_ip_ids,
                ),
                to_pulumi_object_field(
                    "scan_listener_port_tcp",
                    &self.r#scan_listener_port_tcp,
                ),
                to_pulumi_object_field(
                    "scan_listener_port_tcp_ssl",
                    &self.r#scan_listener_port_tcp_ssl,
                ),
                to_pulumi_object_field(
                    "shape",
                    &self.r#shape,
                ),
                to_pulumi_object_field(
                    "sparse_diskgroup_enabled",
                    &self.r#sparse_diskgroup_enabled,
                ),
                to_pulumi_object_field(
                    "ssh_public_keys",
                    &self.r#ssh_public_keys,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "storage_size_gb",
                    &self.r#storage_size_gb,
                ),
                to_pulumi_object_field(
                    "system_version",
                    &self.r#system_version,
                ),
                to_pulumi_object_field(
                    "time_zones",
                    &self.r#time_zones,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCloudVmClustersCloudVmClusterProperty {
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
                    r#cluster_name: {
                        let field_value = match fields_map.get("cluster_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#db_server_ocids: {
                        let field_value = match fields_map.get("db_server_ocids") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_server_ocids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#diagnostics_data_collection_options: {
                        let field_value = match fields_map.get("diagnostics_data_collection_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'diagnostics_data_collection_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_redundancy: {
                        let field_value = match fields_map.get("disk_redundancy") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_redundancy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_listener_ip: {
                        let field_value = match fields_map.get("dns_listener_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_listener_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain: {
                        let field_value = match fields_map.get("domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gi_version: {
                        let field_value = match fields_map.get("gi_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'gi_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#hostname_prefix: {
                        let field_value = match fields_map.get("hostname_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostname_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#license_type: {
                        let field_value = match fields_map.get("license_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'license_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_backup_enabled: {
                        let field_value = match fields_map.get("local_backup_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_backup_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#node_count: {
                        let field_value = match fields_map.get("node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ocpu_count: {
                        let field_value = match fields_map.get("ocpu_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'ocpu_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_dns: {
                        let field_value = match fields_map.get("scan_dns") {
                            Some(value) => value,
                            None => bail!("Missing field 'scan_dns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_dns_record_id: {
                        let field_value = match fields_map.get("scan_dns_record_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'scan_dns_record_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_ip_ids: {
                        let field_value = match fields_map.get("scan_ip_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'scan_ip_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_listener_port_tcp: {
                        let field_value = match fields_map.get("scan_listener_port_tcp") {
                            Some(value) => value,
                            None => bail!("Missing field 'scan_listener_port_tcp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_listener_port_tcp_ssl: {
                        let field_value = match fields_map.get("scan_listener_port_tcp_ssl") {
                            Some(value) => value,
                            None => bail!("Missing field 'scan_listener_port_tcp_ssl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#sparse_diskgroup_enabled: {
                        let field_value = match fields_map.get("sparse_diskgroup_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'sparse_diskgroup_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_public_keys: {
                        let field_value = match fields_map.get("ssh_public_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_public_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#storage_size_gb: {
                        let field_value = match fields_map.get("storage_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#system_version: {
                        let field_value = match fields_map.get("system_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'system_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_zones: {
                        let field_value = match fields_map.get("time_zones") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_zones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
