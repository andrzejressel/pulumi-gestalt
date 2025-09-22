#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterConfigWorkerConfigDiskConfig {
    /// Size of the primary disk attached to each worker node, specified
    /// in GB. The smallest allowed disk size is 10GB. GCP will default to a predetermined
    /// computed value if not set (currently 500GB). Note: If SSDs are not
    /// attached, it also contains the HDFS data blocks and Hadoop working directories.
    #[builder(into)]
    #[serde(rename = "bootDiskSizeGb")]
    pub r#boot_disk_size_gb: Option<i32>,
    /// The disk type of the primary disk attached to each node.
    /// One of `"pd-ssd"` or `"pd-standard"`. Defaults to `"pd-standard"`.
    #[builder(into)]
    #[serde(rename = "bootDiskType")]
    pub r#boot_disk_type: Option<String>,
    /// Interface type of local SSDs (default is "scsi"). Valid values: "scsi" (Small Computer System Interface), "nvme" (Non-Volatile Memory Express).
    #[builder(into)]
    #[serde(rename = "localSsdInterface")]
    pub r#local_ssd_interface: Option<String>,
    /// The amount of local SSD disks that will be
    /// attached to each worker cluster node. Defaults to 0.
    #[builder(into)]
    #[serde(rename = "numLocalSsds")]
    pub r#num_local_ssds: Option<i32>,
}
