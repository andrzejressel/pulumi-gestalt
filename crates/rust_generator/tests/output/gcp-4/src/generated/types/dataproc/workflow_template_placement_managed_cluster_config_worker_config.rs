#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplatePlacementManagedClusterConfigWorkerConfig {
    /// Optional. The Compute Engine accelerator configuration for these instances.
    #[builder(into)]
    #[serde(rename = "accelerators")]
    pub r#accelerators: Option<Vec<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigWorkerConfigAccelerator>>,
    /// Optional. Disk option config settings.
    #[builder(into)]
    #[serde(rename = "diskConfig")]
    pub r#disk_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigWorkerConfigDiskConfig>>,
    /// Optional. The Compute Engine image resource used for cluster instances. The URI can represent an image or image family. Image examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/[image-id]` * `projects/[project_id]/global/images/[image-id]` * `image-id` Image family examples. Dataproc will use the most recent image from the family: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/family/[custom-image-family-name]` * `projects/[project_id]/global/images/family/[custom-image-family-name]` If the URI is unspecified, it will be inferred from `SoftwareConfig.image_version` or the system default.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Option<String>,
    /// Output only. The list of instance names. Dataproc derives the names from `cluster_name`, `num_instances`, and the instance group.
    #[builder(into)]
    #[serde(rename = "instanceNames")]
    pub r#instance_names: Option<Vec<String>>,
    /// Output only. Specifies that this instance group contains preemptible instances.
    #[builder(into)]
    #[serde(rename = "isPreemptible")]
    pub r#is_preemptible: Option<bool>,
    /// Optional. The Compute Engine machine type used for cluster instances. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `n1-standard-2` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the machine type resource, for example, `n1-standard-2`.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Option<String>,
    /// Output only. The config for Compute Engine Instance Group Manager that manages this group. This is only used for preemptible instance groups.
    #[builder(into)]
    #[serde(rename = "managedGroupConfigs")]
    pub r#managed_group_configs: Option<Vec<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigWorkerConfigManagedGroupConfig>>,
    /// Optional. Specifies the minimum cpu platform for the Instance Group. See [Dataproc > Minimum CPU Platform](https://cloud.google.com/dataproc/docs/concepts/compute/dataproc-min-cpu).
    #[builder(into)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Option<String>,
    /// Optional. The number of VM instances in the instance group. For [HA cluster](https://www.terraform.io/dataproc/docs/concepts/configuring-clusters/high-availability) master_config groups, **must be set to 3**. For standard cluster master_config groups, **must be set to 1**.
    #[builder(into)]
    #[serde(rename = "numInstances")]
    pub r#num_instances: Option<i32>,
    /// Optional. Specifies the preemptibility of the instance group. The default value for master and worker groups is `NON_PREEMPTIBLE`. This default cannot be changed. The default value for secondary instances is `PREEMPTIBLE`. Possible values: PREEMPTIBILITY_UNSPECIFIED, NON_PREEMPTIBLE, PREEMPTIBLE
    #[builder(into)]
    #[serde(rename = "preemptibility")]
    pub r#preemptibility: Option<String>,
}
