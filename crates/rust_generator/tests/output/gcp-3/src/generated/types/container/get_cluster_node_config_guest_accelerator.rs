#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterNodeConfigGuestAccelerator {
    /// The number of the accelerator cards exposed to an instance.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    /// Configuration for auto installation of GPU driver.
    #[builder(into)]
    #[serde(rename = "gpuDriverInstallationConfigs")]
    pub r#gpu_driver_installation_configs: Vec<super::super::types::container::GetClusterNodeConfigGuestAcceleratorGpuDriverInstallationConfig>,
    /// Size of partitions to create on the GPU. Valid values are described in the NVIDIA mig user guide (https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning)
    #[builder(into)]
    #[serde(rename = "gpuPartitionSize")]
    pub r#gpu_partition_size: String,
    /// Configuration for GPU sharing.
    #[builder(into)]
    #[serde(rename = "gpuSharingConfigs")]
    pub r#gpu_sharing_configs: Vec<super::super::types::container::GetClusterNodeConfigGuestAcceleratorGpuSharingConfig>,
    /// The accelerator type resource name.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
