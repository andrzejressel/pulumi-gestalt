#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CloudVmClusterProperties {
    /// OCI Cluster name.
    #[builder(into)]
    pub r#cluster_name: Option<String>,
    /// (Output)
    /// Compartment ID of cluster.
    #[builder(into)]
    pub r#compartment_id: Option<String>,
    /// Number of enabled CPU cores.
    #[builder(into)]
    pub r#cpu_core_count: i32,
    /// The data disk group size to be allocated in TBs.
    #[builder(into)]
    pub r#data_storage_size_tb: Option<f64>,
    /// Local storage per VM
    #[builder(into)]
    pub r#db_node_storage_size_gb: Option<i32>,
    /// OCID of database servers.
    #[builder(into)]
    pub r#db_server_ocids: Option<Vec<String>>,
    /// Data collection options for diagnostics.
    /// Structure is documented below.
    #[builder(into)]
    pub r#diagnostics_data_collection_options: Option<Box<super::super::types::oracledatabase::CloudVmClusterPropertiesDiagnosticsDataCollectionOptions>>,
    /// The type of redundancy.
    /// Possible values:
    /// DISK_REDUNDANCY_UNSPECIFIED
    /// HIGH
    /// NORMAL
    #[builder(into)]
    pub r#disk_redundancy: Option<String>,
    /// (Output)
    /// DNS listener IP.
    #[builder(into)]
    pub r#dns_listener_ip: Option<String>,
    /// (Output)
    /// Parent DNS domain where SCAN DNS and hosts names are qualified.
    /// ex: ocispdelegated.ocisp10jvnet.oraclevcn.com
    #[builder(into)]
    pub r#domain: Option<String>,
    /// Grid Infrastructure Version.
    #[builder(into)]
    pub r#gi_version: Option<String>,
    /// (Output)
    /// host name without domain.
    /// format: "-" with some suffix.
    /// ex: sp2-yi0xq where "sp2" is the hostname_prefix.
    #[builder(into)]
    pub r#hostname: Option<String>,
    /// Prefix for VM cluster host names.
    #[builder(into)]
    pub r#hostname_prefix: Option<String>,
    /// License type of VM Cluster.
    /// Possible values:
    /// LICENSE_TYPE_UNSPECIFIED
    /// LICENSE_INCLUDED
    /// BRING_YOUR_OWN_LICENSE
    #[builder(into)]
    pub r#license_type: String,
    /// Use local backup.
    #[builder(into)]
    pub r#local_backup_enabled: Option<bool>,
    /// Memory allocated in GBs.
    #[builder(into)]
    pub r#memory_size_gb: Option<i32>,
    /// Number of database servers.
    #[builder(into)]
    pub r#node_count: Option<i32>,
    /// (Output)
    /// Deep link to the OCI console to view this resource.
    #[builder(into)]
    pub r#oci_url: Option<String>,
    /// (Output)
    /// Oracle Cloud Infrastructure ID of VM Cluster.
    #[builder(into)]
    pub r#ocid: Option<String>,
    /// OCPU count per VM. Minimum is 0.1.
    #[builder(into)]
    pub r#ocpu_count: Option<f64>,
    /// (Output)
    /// SCAN DNS name.
    /// ex: sp2-yi0xq-scan.ocispdelegated.ocisp10jvnet.oraclevcn.com
    #[builder(into)]
    pub r#scan_dns: Option<String>,
    /// (Output)
    /// OCID of scan DNS record.
    #[builder(into)]
    pub r#scan_dns_record_id: Option<String>,
    /// (Output)
    /// OCIDs of scan IPs.
    #[builder(into)]
    pub r#scan_ip_ids: Option<Vec<String>>,
    /// (Output)
    /// SCAN listener port - TCP
    #[builder(into)]
    pub r#scan_listener_port_tcp: Option<i32>,
    /// (Output)
    /// SCAN listener port - TLS
    #[builder(into)]
    pub r#scan_listener_port_tcp_ssl: Option<i32>,
    /// (Output)
    /// Shape of VM Cluster.
    #[builder(into)]
    pub r#shape: Option<String>,
    /// Use exadata sparse snapshots.
    #[builder(into)]
    pub r#sparse_diskgroup_enabled: Option<bool>,
    /// SSH public keys to be stored with cluster.
    #[builder(into)]
    pub r#ssh_public_keys: Option<Vec<String>>,
    /// (Output)
    /// State of the cluster.
    /// Possible values:
    /// STATE_UNSPECIFIED
    /// PROVISIONING
    /// AVAILABLE
    /// UPDATING
    /// TERMINATING
    /// TERMINATED
    /// FAILED
    /// MAINTENANCE_IN_PROGRESS
    #[builder(into)]
    pub r#state: Option<String>,
    /// (Output)
    /// The storage allocation for the disk group, in gigabytes (GB).
    #[builder(into)]
    pub r#storage_size_gb: Option<i32>,
    /// (Output)
    /// Operating system version of the image.
    #[builder(into)]
    pub r#system_version: Option<String>,
    /// Represents a time zone from the
    /// [IANA Time Zone Database](https://www.iana.org/time-zones).
    /// Structure is documented below.
    #[builder(into)]
    pub r#time_zone: Option<Box<super::super::types::oracledatabase::CloudVmClusterPropertiesTimeZone>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CloudVmClusterProperties {
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
                    "clusterName",
                    &self.r#cluster_name,
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
                    "dataStorageSizeTb",
                    &self.r#data_storage_size_tb,
                ),
                to_pulumi_object_field(
                    "dbNodeStorageSizeGb",
                    &self.r#db_node_storage_size_gb,
                ),
                to_pulumi_object_field(
                    "dbServerOcids",
                    &self.r#db_server_ocids,
                ),
                to_pulumi_object_field(
                    "diagnosticsDataCollectionOptions",
                    &self.r#diagnostics_data_collection_options,
                ),
                to_pulumi_object_field(
                    "diskRedundancy",
                    &self.r#disk_redundancy,
                ),
                to_pulumi_object_field(
                    "dnsListenerIp",
                    &self.r#dns_listener_ip,
                ),
                to_pulumi_object_field(
                    "domain",
                    &self.r#domain,
                ),
                to_pulumi_object_field(
                    "giVersion",
                    &self.r#gi_version,
                ),
                to_pulumi_object_field(
                    "hostname",
                    &self.r#hostname,
                ),
                to_pulumi_object_field(
                    "hostnamePrefix",
                    &self.r#hostname_prefix,
                ),
                to_pulumi_object_field(
                    "licenseType",
                    &self.r#license_type,
                ),
                to_pulumi_object_field(
                    "localBackupEnabled",
                    &self.r#local_backup_enabled,
                ),
                to_pulumi_object_field(
                    "memorySizeGb",
                    &self.r#memory_size_gb,
                ),
                to_pulumi_object_field(
                    "nodeCount",
                    &self.r#node_count,
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
                    "ocpuCount",
                    &self.r#ocpu_count,
                ),
                to_pulumi_object_field(
                    "scanDns",
                    &self.r#scan_dns,
                ),
                to_pulumi_object_field(
                    "scanDnsRecordId",
                    &self.r#scan_dns_record_id,
                ),
                to_pulumi_object_field(
                    "scanIpIds",
                    &self.r#scan_ip_ids,
                ),
                to_pulumi_object_field(
                    "scanListenerPortTcp",
                    &self.r#scan_listener_port_tcp,
                ),
                to_pulumi_object_field(
                    "scanListenerPortTcpSsl",
                    &self.r#scan_listener_port_tcp_ssl,
                ),
                to_pulumi_object_field(
                    "shape",
                    &self.r#shape,
                ),
                to_pulumi_object_field(
                    "sparseDiskgroupEnabled",
                    &self.r#sparse_diskgroup_enabled,
                ),
                to_pulumi_object_field(
                    "sshPublicKeys",
                    &self.r#ssh_public_keys,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "storageSizeGb",
                    &self.r#storage_size_gb,
                ),
                to_pulumi_object_field(
                    "systemVersion",
                    &self.r#system_version,
                ),
                to_pulumi_object_field(
                    "timeZone",
                    &self.r#time_zone,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CloudVmClusterProperties {
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
                        let field_value = match fields_map.get("clusterName") {
                            Some(value) => value,
                            None => bail!("Missing field 'clusterName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#db_server_ocids: {
                        let field_value = match fields_map.get("dbServerOcids") {
                            Some(value) => value,
                            None => bail!("Missing field 'dbServerOcids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#diagnostics_data_collection_options: {
                        let field_value = match fields_map.get("diagnosticsDataCollectionOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'diagnosticsDataCollectionOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_redundancy: {
                        let field_value = match fields_map.get("diskRedundancy") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskRedundancy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_listener_ip: {
                        let field_value = match fields_map.get("dnsListenerIp") {
                            Some(value) => value,
                            None => bail!("Missing field 'dnsListenerIp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("giVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'giVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("hostnamePrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostnamePrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#license_type: {
                        let field_value = match fields_map.get("licenseType") {
                            Some(value) => value,
                            None => bail!("Missing field 'licenseType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_backup_enabled: {
                        let field_value = match fields_map.get("localBackupEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'localBackupEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#node_count: {
                        let field_value = match fields_map.get("nodeCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ocpu_count: {
                        let field_value = match fields_map.get("ocpuCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'ocpuCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_dns: {
                        let field_value = match fields_map.get("scanDns") {
                            Some(value) => value,
                            None => bail!("Missing field 'scanDns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_dns_record_id: {
                        let field_value = match fields_map.get("scanDnsRecordId") {
                            Some(value) => value,
                            None => bail!("Missing field 'scanDnsRecordId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_ip_ids: {
                        let field_value = match fields_map.get("scanIpIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'scanIpIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_listener_port_tcp: {
                        let field_value = match fields_map.get("scanListenerPortTcp") {
                            Some(value) => value,
                            None => bail!("Missing field 'scanListenerPortTcp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_listener_port_tcp_ssl: {
                        let field_value = match fields_map.get("scanListenerPortTcpSsl") {
                            Some(value) => value,
                            None => bail!("Missing field 'scanListenerPortTcpSsl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("sparseDiskgroupEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'sparseDiskgroupEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_public_keys: {
                        let field_value = match fields_map.get("sshPublicKeys") {
                            Some(value) => value,
                            None => bail!("Missing field 'sshPublicKeys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("storageSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#system_version: {
                        let field_value = match fields_map.get("systemVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'systemVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_zone: {
                        let field_value = match fields_map.get("timeZone") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeZone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
