#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterDefaultNodePool {
    /// Should [the Kubernetes Auto Scaler](https://docs.microsoft.com/azure/aks/cluster-autoscaler) be enabled for this Node Pool?
    /// 
    /// > **Note:** This requires that the `type` is set to `VirtualMachineScaleSets`.
    /// 
    /// > **Note:** If you're using AutoScaling, you may wish to use [`ignoreChanges` functionality](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) to ignore changes to the `node_count` field.
    #[builder(into)]
    pub r#auto_scaling_enabled: Option<bool>,
    /// Specifies the ID of the Capacity Reservation Group within which this AKS Cluster should be created. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#capacity_reservation_group_id: Option<String>,
    /// Should the nodes in this Node Pool have Federal Information Processing Standard enabled? `temporary_name_for_rotation` must be specified when changing this block. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#fips_enabled: Option<bool>,
    /// Specifies the GPU MIG instance profile for supported GPU VM SKU. The allowed values are `MIG1g`, `MIG2g`, `MIG3g`, `MIG4g` and `MIG7g`. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#gpu_instance: Option<String>,
    /// Should the nodes in the Default Node Pool have host encryption enabled? `temporary_name_for_rotation` must be specified when changing this property.
    /// 
    /// > **Note:** This requires that the  Feature `Microsoft.ContainerService/EnableEncryptionAtHost` is enabled and the Resource Provider is registered.
    #[builder(into)]
    pub r#host_encryption_enabled: Option<bool>,
    /// Specifies the ID of the Host Group within which this AKS Cluster should be created. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#host_group_id: Option<String>,
    /// A `kubelet_config` block as defined below. `temporary_name_for_rotation` must be specified when changing this block.
    #[builder(into)]
    pub r#kubelet_config: Option<Box<super::super::types::containerservice::KubernetesClusterDefaultNodePoolKubeletConfig>>,
    /// The type of disk used by kubelet. Possible values are `OS` and `Temporary`.
    #[builder(into)]
    pub r#kubelet_disk_type: Option<String>,
    /// A `linux_os_config` block as defined below. `temporary_name_for_rotation` must be specified when changing this block.
    #[builder(into)]
    pub r#linux_os_config: Option<Box<super::super::types::containerservice::KubernetesClusterDefaultNodePoolLinuxOsConfig>>,
    #[builder(into)]
    pub r#max_count: Option<i32>,
    /// The maximum number of pods that can run on each agent. `temporary_name_for_rotation` must be specified when changing this property.
    #[builder(into)]
    pub r#max_pods: Option<i32>,
    #[builder(into)]
    pub r#min_count: Option<i32>,
    /// The name which should be used for the default Kubernetes Node Pool.
    #[builder(into)]
    pub r#name: String,
    #[builder(into)]
    pub r#node_count: Option<i32>,
    /// A map of Kubernetes labels which should be applied to nodes in the Default Node Pool.
    #[builder(into)]
    pub r#node_labels: Option<std::collections::BTreeMap<String, String>>,
    /// A `node_network_profile` block as documented below.
    #[builder(into)]
    pub r#node_network_profile: Option<Box<super::super::types::containerservice::KubernetesClusterDefaultNodePoolNodeNetworkProfile>>,
    /// Should nodes in this Node Pool have a Public IP Address? `temporary_name_for_rotation` must be specified when changing this property.
    #[builder(into)]
    pub r#node_public_ip_enabled: Option<bool>,
    /// Resource ID for the Public IP Addresses Prefix for the nodes in this Node Pool. `node_public_ip_enabled` should be `true`. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#node_public_ip_prefix_id: Option<String>,
    /// Enabling this option will taint default node pool with `CriticalAddonsOnly=true:NoSchedule` taint. `temporary_name_for_rotation` must be specified when changing this property.
    #[builder(into)]
    pub r#only_critical_addons_enabled: Option<bool>,
    /// Version of Kubernetes used for the Agents. If not specified, the default node pool will be created with the version specified by `kubernetes_version`. If both are unspecified, the latest recommended version will be used at provisioning time (but won't auto-upgrade). AKS does not require an exact patch version to be specified, minor version aliases such as `1.22` are also supported. - The minor version's latest GA patch is automatically chosen in that case. More details can be found in [the documentation](https://docs.microsoft.com/en-us/azure/aks/supported-kubernetes-versions?tabs=azure-cli#alias-minor-version).
    /// 
    /// > **Note:** This version must be supported by the Kubernetes Cluster - as such the version of Kubernetes used on the Cluster/Control Plane may need to be upgraded first.
    #[builder(into)]
    pub r#orchestrator_version: Option<String>,
    /// The size of the OS Disk which should be used for each agent in the Node Pool. `temporary_name_for_rotation` must be specified when attempting a change.
    #[builder(into)]
    pub r#os_disk_size_gb: Option<i32>,
    /// The type of disk which should be used for the Operating System. Possible values are `Ephemeral` and `Managed`. Defaults to `Managed`. `temporary_name_for_rotation` must be specified when attempting a change.
    #[builder(into)]
    pub r#os_disk_type: Option<String>,
    /// Specifies the OS SKU used by the agent pool. Possible values are `AzureLinux`, `Ubuntu`, `Windows2019` and `Windows2022`. If not specified, the default is `Ubuntu` if OSType=Linux or `Windows2019` if OSType=Windows. And the default Windows OSSKU will be changed to `Windows2022` after Windows2019 is deprecated. Changing this from `AzureLinux` or `Ubuntu` to `AzureLinux` or `Ubuntu` will not replace the resource, otherwise `temporary_name_for_rotation` must be specified when attempting a change.
    #[builder(into)]
    pub r#os_sku: Option<String>,
    /// The ID of the Subnet where the pods in the default Node Pool should exist.
    #[builder(into)]
    pub r#pod_subnet_id: Option<String>,
    /// The ID of the Proximity Placement Group. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#proximity_placement_group_id: Option<String>,
    /// Specifies the autoscaling behaviour of the Kubernetes Cluster. Allowed values are `Delete` and `Deallocate`. Defaults to `Delete`.
    #[builder(into)]
    pub r#scale_down_mode: Option<String>,
    /// The ID of the Snapshot which should be used to create this default Node Pool. `temporary_name_for_rotation` must be specified when changing this property.
    #[builder(into)]
    pub r#snapshot_id: Option<String>,
    /// A mapping of tags to assign to the Node Pool.
    /// 
    /// > At this time there's a bug in the AKS API where Tags for a Node Pool are not stored in the correct case - you may wish to use `ignore_changes` functionality to ignore changes to the casing until this is fixed in the AKS API.
    #[builder(into)]
    pub r#tags: Option<std::collections::BTreeMap<String, String>>,
    /// Specifies the name of the temporary node pool used to cycle the default node pool for VM resizing.
    #[builder(into)]
    pub r#temporary_name_for_rotation: Option<String>,
    /// The type of Node Pool which should be created. Possible values are `VirtualMachineScaleSets`. Defaults to `VirtualMachineScaleSets`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** When creating a cluster that supports multiple node pools, the cluster must use `VirtualMachineScaleSets`. For more information on the limitations of clusters using multiple node pools see [the documentation](https://learn.microsoft.com/en-us/azure/aks/use-multiple-node-pools#limitations).
    #[builder(into)]
    pub r#type_: Option<String>,
    /// Used to specify whether the UltraSSD is enabled in the Default Node Pool. Defaults to `false`. See [the documentation](https://docs.microsoft.com/azure/aks/use-ultra-disks) for more information. `temporary_name_for_rotation` must be specified when attempting a change.
    #[builder(into)]
    pub r#ultra_ssd_enabled: Option<bool>,
    /// A `upgrade_settings` block as documented below.
    #[builder(into)]
    pub r#upgrade_settings: Option<Box<super::super::types::containerservice::KubernetesClusterDefaultNodePoolUpgradeSettings>>,
    /// The size of the Virtual Machine, such as `Standard_DS2_v2`. `temporary_name_for_rotation` must be specified when attempting a resize.
    #[builder(into)]
    pub r#vm_size: String,
    /// The ID of a Subnet where the Kubernetes Node Pool should exist.
    /// 
    /// > **Note:** A Route Table must be configured on this Subnet.
    #[builder(into)]
    pub r#vnet_subnet_id: Option<String>,
    /// Specifies the workload runtime used by the node pool. Possible value is `OCIContainer`.
    #[builder(into)]
    pub r#workload_runtime: Option<String>,
    /// Specifies a list of Availability Zones in which this Kubernetes Cluster should be located. `temporary_name_for_rotation` must be specified when changing this property.
    /// 
    /// > **Note:** This requires that the `type` is set to `VirtualMachineScaleSets` and that `load_balancer_sku` is set to `standard`.
    #[builder(into)]
    pub r#zones: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterDefaultNodePool {
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
                    "autoScalingEnabled",
                    &self.r#auto_scaling_enabled,
                ),
                to_pulumi_object_field(
                    "capacityReservationGroupId",
                    &self.r#capacity_reservation_group_id,
                ),
                to_pulumi_object_field(
                    "fipsEnabled",
                    &self.r#fips_enabled,
                ),
                to_pulumi_object_field(
                    "gpuInstance",
                    &self.r#gpu_instance,
                ),
                to_pulumi_object_field(
                    "hostEncryptionEnabled",
                    &self.r#host_encryption_enabled,
                ),
                to_pulumi_object_field(
                    "hostGroupId",
                    &self.r#host_group_id,
                ),
                to_pulumi_object_field(
                    "kubeletConfig",
                    &self.r#kubelet_config,
                ),
                to_pulumi_object_field(
                    "kubeletDiskType",
                    &self.r#kubelet_disk_type,
                ),
                to_pulumi_object_field(
                    "linuxOsConfig",
                    &self.r#linux_os_config,
                ),
                to_pulumi_object_field(
                    "maxCount",
                    &self.r#max_count,
                ),
                to_pulumi_object_field(
                    "maxPods",
                    &self.r#max_pods,
                ),
                to_pulumi_object_field(
                    "minCount",
                    &self.r#min_count,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "nodeCount",
                    &self.r#node_count,
                ),
                to_pulumi_object_field(
                    "nodeLabels",
                    &self.r#node_labels,
                ),
                to_pulumi_object_field(
                    "nodeNetworkProfile",
                    &self.r#node_network_profile,
                ),
                to_pulumi_object_field(
                    "nodePublicIpEnabled",
                    &self.r#node_public_ip_enabled,
                ),
                to_pulumi_object_field(
                    "nodePublicIpPrefixId",
                    &self.r#node_public_ip_prefix_id,
                ),
                to_pulumi_object_field(
                    "onlyCriticalAddonsEnabled",
                    &self.r#only_critical_addons_enabled,
                ),
                to_pulumi_object_field(
                    "orchestratorVersion",
                    &self.r#orchestrator_version,
                ),
                to_pulumi_object_field(
                    "osDiskSizeGb",
                    &self.r#os_disk_size_gb,
                ),
                to_pulumi_object_field(
                    "osDiskType",
                    &self.r#os_disk_type,
                ),
                to_pulumi_object_field(
                    "osSku",
                    &self.r#os_sku,
                ),
                to_pulumi_object_field(
                    "podSubnetId",
                    &self.r#pod_subnet_id,
                ),
                to_pulumi_object_field(
                    "proximityPlacementGroupId",
                    &self.r#proximity_placement_group_id,
                ),
                to_pulumi_object_field(
                    "scaleDownMode",
                    &self.r#scale_down_mode,
                ),
                to_pulumi_object_field(
                    "snapshotId",
                    &self.r#snapshot_id,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "temporaryNameForRotation",
                    &self.r#temporary_name_for_rotation,
                ),
                to_pulumi_object_field(
                    "type",
                    &self.r#type_,
                ),
                to_pulumi_object_field(
                    "ultraSsdEnabled",
                    &self.r#ultra_ssd_enabled,
                ),
                to_pulumi_object_field(
                    "upgradeSettings",
                    &self.r#upgrade_settings,
                ),
                to_pulumi_object_field(
                    "vmSize",
                    &self.r#vm_size,
                ),
                to_pulumi_object_field(
                    "vnetSubnetId",
                    &self.r#vnet_subnet_id,
                ),
                to_pulumi_object_field(
                    "workloadRuntime",
                    &self.r#workload_runtime,
                ),
                to_pulumi_object_field(
                    "zones",
                    &self.r#zones,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterDefaultNodePool {
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
                    r#auto_scaling_enabled: {
                        let field_value = match fields_map.get("autoScalingEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoScalingEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#capacity_reservation_group_id: {
                        let field_value = match fields_map.get("capacityReservationGroupId") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacityReservationGroupId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fips_enabled: {
                        let field_value = match fields_map.get("fipsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'fipsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gpu_instance: {
                        let field_value = match fields_map.get("gpuInstance") {
                            Some(value) => value,
                            None => bail!("Missing field 'gpuInstance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_encryption_enabled: {
                        let field_value = match fields_map.get("hostEncryptionEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostEncryptionEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_group_id: {
                        let field_value = match fields_map.get("hostGroupId") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostGroupId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kubelet_config: {
                        let field_value = match fields_map.get("kubeletConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'kubeletConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kubelet_disk_type: {
                        let field_value = match fields_map.get("kubeletDiskType") {
                            Some(value) => value,
                            None => bail!("Missing field 'kubeletDiskType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linux_os_config: {
                        let field_value = match fields_map.get("linuxOsConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'linuxOsConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_count: {
                        let field_value = match fields_map.get("maxCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_pods: {
                        let field_value = match fields_map.get("maxPods") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxPods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_count: {
                        let field_value = match fields_map.get("minCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'minCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#node_labels: {
                        let field_value = match fields_map.get("nodeLabels") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeLabels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_network_profile: {
                        let field_value = match fields_map.get("nodeNetworkProfile") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeNetworkProfile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_public_ip_enabled: {
                        let field_value = match fields_map.get("nodePublicIpEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodePublicIpEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_public_ip_prefix_id: {
                        let field_value = match fields_map.get("nodePublicIpPrefixId") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodePublicIpPrefixId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#only_critical_addons_enabled: {
                        let field_value = match fields_map.get("onlyCriticalAddonsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'onlyCriticalAddonsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#orchestrator_version: {
                        let field_value = match fields_map.get("orchestratorVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'orchestratorVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_disk_size_gb: {
                        let field_value = match fields_map.get("osDiskSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'osDiskSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_disk_type: {
                        let field_value = match fields_map.get("osDiskType") {
                            Some(value) => value,
                            None => bail!("Missing field 'osDiskType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_sku: {
                        let field_value = match fields_map.get("osSku") {
                            Some(value) => value,
                            None => bail!("Missing field 'osSku' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_subnet_id: {
                        let field_value = match fields_map.get("podSubnetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'podSubnetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proximity_placement_group_id: {
                        let field_value = match fields_map.get("proximityPlacementGroupId") {
                            Some(value) => value,
                            None => bail!("Missing field 'proximityPlacementGroupId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_down_mode: {
                        let field_value = match fields_map.get("scaleDownMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaleDownMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot_id: {
                        let field_value = match fields_map.get("snapshotId") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshotId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#temporary_name_for_rotation: {
                        let field_value = match fields_map.get("temporaryNameForRotation") {
                            Some(value) => value,
                            None => bail!("Missing field 'temporaryNameForRotation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type") {
                            Some(value) => value,
                            None => bail!("Missing field 'type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ultra_ssd_enabled: {
                        let field_value = match fields_map.get("ultraSsdEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ultraSsdEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upgrade_settings: {
                        let field_value = match fields_map.get("upgradeSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'upgradeSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_size: {
                        let field_value = match fields_map.get("vmSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnet_subnet_id: {
                        let field_value = match fields_map.get("vnetSubnetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnetSubnetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workload_runtime: {
                        let field_value = match fields_map.get("workloadRuntime") {
                            Some(value) => value,
                            None => bail!("Missing field 'workloadRuntime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zones: {
                        let field_value = match fields_map.get("zones") {
                            Some(value) => value,
                            None => bail!("Missing field 'zones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
