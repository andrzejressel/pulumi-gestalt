#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CloudExadataInfrastructureProperties {
    /// (Output)
    /// The requested number of additional storage servers activated for the
    /// Exadata Infrastructure.
    #[builder(into)]
    #[serde(rename = "activatedStorageCount")]
    pub r#activated_storage_count: Option<i32>,
    /// (Output)
    /// The requested number of additional storage servers for the Exadata
    /// Infrastructure.
    #[builder(into)]
    #[serde(rename = "additionalStorageCount")]
    pub r#additional_storage_count: Option<i32>,
    /// (Output)
    /// The available storage can be allocated to the Exadata Infrastructure
    /// resource, in gigabytes (GB).
    #[builder(into)]
    #[serde(rename = "availableStorageSizeGb")]
    pub r#available_storage_size_gb: Option<i32>,
    /// The number of compute servers for the Exadata Infrastructure.
    #[builder(into)]
    #[serde(rename = "computeCount")]
    pub r#compute_count: Option<i32>,
    /// (Output)
    /// The number of enabled CPU cores.
    #[builder(into)]
    #[serde(rename = "cpuCount")]
    pub r#cpu_count: Option<i32>,
    /// The list of customer contacts.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customerContacts")]
    pub r#customer_contacts: Option<Vec<super::super::types::oracledatabase::CloudExadataInfrastructurePropertiesCustomerContact>>,
    /// (Output)
    /// Size, in terabytes, of the DATA disk group.
    #[builder(into)]
    #[serde(rename = "dataStorageSizeTb")]
    pub r#data_storage_size_tb: Option<f64>,
    /// (Output)
    /// The local node storage allocated in GBs.
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeGb")]
    pub r#db_node_storage_size_gb: Option<i32>,
    /// (Output)
    /// The software version of the database servers (dom0) in the Exadata
    /// Infrastructure.
    #[builder(into)]
    #[serde(rename = "dbServerVersion")]
    pub r#db_server_version: Option<String>,
    /// Maintenance window as defined by Oracle.
    /// https://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/MaintenanceWindow
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "maintenanceWindow")]
    pub r#maintenance_window: Box<Option<super::super::types::oracledatabase::CloudExadataInfrastructurePropertiesMaintenanceWindow>>,
    /// (Output)
    /// The total number of CPU cores available.
    #[builder(into)]
    #[serde(rename = "maxCpuCount")]
    pub r#max_cpu_count: Option<i32>,
    /// (Output)
    /// The total available DATA disk group size.
    #[builder(into)]
    #[serde(rename = "maxDataStorageTb")]
    pub r#max_data_storage_tb: Option<f64>,
    /// (Output)
    /// The total local node storage available in GBs.
    #[builder(into)]
    #[serde(rename = "maxDbNodeStorageSizeGb")]
    pub r#max_db_node_storage_size_gb: Option<i32>,
    /// (Output)
    /// The total memory available in GBs.
    #[builder(into)]
    #[serde(rename = "maxMemoryGb")]
    pub r#max_memory_gb: Option<i32>,
    /// (Output)
    /// The memory allocated in GBs.
    #[builder(into)]
    #[serde(rename = "memorySizeGb")]
    pub r#memory_size_gb: Option<i32>,
    /// (Output)
    /// The monthly software version of the database servers (dom0)
    /// in the Exadata Infrastructure. Example: 20.1.15
    #[builder(into)]
    #[serde(rename = "monthlyDbServerVersion")]
    pub r#monthly_db_server_version: Option<String>,
    /// (Output)
    /// The monthly software version of the storage servers (cells)
    /// in the Exadata Infrastructure. Example: 20.1.15
    #[builder(into)]
    #[serde(rename = "monthlyStorageServerVersion")]
    pub r#monthly_storage_server_version: Option<String>,
    /// (Output)
    /// The OCID of the next maintenance run.
    #[builder(into)]
    #[serde(rename = "nextMaintenanceRunId")]
    pub r#next_maintenance_run_id: Option<String>,
    /// (Output)
    /// The time when the next maintenance run will occur.
    #[builder(into)]
    #[serde(rename = "nextMaintenanceRunTime")]
    pub r#next_maintenance_run_time: Option<String>,
    /// (Output)
    /// The time when the next security maintenance run will occur.
    #[builder(into)]
    #[serde(rename = "nextSecurityMaintenanceRunTime")]
    pub r#next_security_maintenance_run_time: Option<String>,
    /// (Output)
    /// Deep link to the OCI console to view this resource.
    #[builder(into)]
    #[serde(rename = "ociUrl")]
    pub r#oci_url: Option<String>,
    /// (Output)
    /// OCID of created infra.
    /// https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm#Oracle
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: Option<String>,
    /// The shape of the Exadata Infrastructure. The shape determines the
    /// amount of CPU, storage, and memory resources allocated to the instance.
    #[builder(into)]
    #[serde(rename = "shape")]
    pub r#shape: String,
    /// (Output)
    /// The current lifecycle state of the Exadata Infrastructure.
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
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// The number of Cloud Exadata storage servers for the Exadata Infrastructure.
    #[builder(into)]
    #[serde(rename = "storageCount")]
    pub r#storage_count: Option<i32>,
    /// (Output)
    /// The software version of the storage servers (cells) in the Exadata
    /// Infrastructure.
    #[builder(into)]
    #[serde(rename = "storageServerVersion")]
    pub r#storage_server_version: Option<String>,
    /// The total storage allocated to the Exadata Infrastructure
    /// resource, in gigabytes (GB).
    #[builder(into)]
    #[serde(rename = "totalStorageSizeGb")]
    pub r#total_storage_size_gb: Option<i32>,
}
