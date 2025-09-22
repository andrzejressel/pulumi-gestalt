#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkstationConfigHostGceInstanceBoostConfig {
    /// An accelerator card attached to the boost instance.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "accelerators")]
    pub r#accelerators: Option<Vec<super::super::types::workstations::WorkstationConfigHostGceInstanceBoostConfigAccelerator>>,
    /// Size of the boot disk in GB. The minimum boot disk size is `30` GB. Defaults to `50` GB.
    #[builder(into)]
    #[serde(rename = "bootDiskSizeGb")]
    pub r#boot_disk_size_gb: Option<i32>,
    /// Whether to enable nested virtualization on the Compute Engine VMs backing boosted Workstations.
    /// See https://cloud.google.com/workstations/docs/reference/rest/v1beta/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization
    #[builder(into)]
    #[serde(rename = "enableNestedVirtualization")]
    pub r#enable_nested_virtualization: Option<bool>,
    /// The id to be used for the boost config.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The type of machine that boosted VM instances will use—for example, e2-standard-4. For more information about machine types that Cloud Workstations supports, see the list of available machine types https://cloud.google.com/workstations/docs/available-machine-types. Defaults to e2-standard-4.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Option<String>,
    /// Number of instances to pool for faster workstation boosting.
    #[builder(into)]
    #[serde(rename = "poolSize")]
    pub r#pool_size: Option<i32>,
}
