#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodePoolNodeConfigGuestAccelerator {
    /// The number of the guest accelerator cards exposed to this instance.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    /// Configuration for auto installation of GPU driver. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gpuDriverInstallationConfig")]
    pub r#gpu_driver_installation_config: Box<Option<super::super::types::container::ClusterNodePoolNodeConfigGuestAcceleratorGpuDriverInstallationConfig>>,
    /// Size of partitions to create on the GPU. Valid values are described in the NVIDIA mig [user guide](https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning).
    #[builder(into)]
    #[serde(rename = "gpuPartitionSize")]
    pub r#gpu_partition_size: Option<String>,
    /// Configuration for GPU sharing. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gpuSharingConfig")]
    pub r#gpu_sharing_config: Box<Option<super::super::types::container::ClusterNodePoolNodeConfigGuestAcceleratorGpuSharingConfig>>,
    /// The accelerator type resource to expose to this instance. E.g. `nvidia-tesla-k80`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
