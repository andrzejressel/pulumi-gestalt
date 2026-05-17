#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
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
    #[serde(rename = "autoScalingEnabled")]
    pub r#auto_scaling_enabled: Option<bool>,
    /// Specifies the ID of the Capacity Reservation Group within which this AKS Cluster should be created. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "capacityReservationGroupId")]
    pub r#capacity_reservation_group_id: Option<String>,
    /// Should the nodes in this Node Pool have Federal Information Processing Standard enabled? `temporary_name_for_rotation` must be specified when changing this block. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "fipsEnabled")]
    pub r#fips_enabled: Option<bool>,
    /// Specifies the GPU MIG instance profile for supported GPU VM SKU. The allowed values are `MIG1g`, `MIG2g`, `MIG3g`, `MIG4g` and `MIG7g`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "gpuInstance")]
    pub r#gpu_instance: Option<String>,
    /// Should the nodes in the Default Node Pool have host encryption enabled? `temporary_name_for_rotation` must be specified when changing this property.
    /// 
    /// > **Note:** This requires that the  Feature `Microsoft.ContainerService/EnableEncryptionAtHost` is enabled and the Resource Provider is registered.
    #[builder(into)]
    #[serde(rename = "hostEncryptionEnabled")]
    pub r#host_encryption_enabled: Option<bool>,
    /// Specifies the ID of the Host Group within which this AKS Cluster should be created. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "hostGroupId")]
    pub r#host_group_id: Option<String>,
    /// A `kubelet_config` block as defined below. `temporary_name_for_rotation` must be specified when changing this block.
    #[builder(into)]
    #[serde(rename = "kubeletConfig")]
    pub r#kubelet_config: Option<Box<super::super::types::containerservice::KubernetesClusterDefaultNodePoolKubeletConfig>>,
    /// The type of disk used by kubelet. Possible values are `OS` and `Temporary`.
    #[builder(into)]
    #[serde(rename = "kubeletDiskType")]
    pub r#kubelet_disk_type: Option<String>,
    /// A `linux_os_config` block as defined below. `temporary_name_for_rotation` must be specified when changing this block.
    #[builder(into)]
    #[serde(rename = "linuxOsConfig")]
    pub r#linux_os_config: Option<Box<super::super::types::containerservice::KubernetesClusterDefaultNodePoolLinuxOsConfig>>,
    #[builder(into)]
    #[serde(rename = "maxCount")]
    pub r#max_count: Option<i32>,
    /// The maximum number of pods that can run on each agent. `temporary_name_for_rotation` must be specified when changing this property.
    #[builder(into)]
    #[serde(rename = "maxPods")]
    pub r#max_pods: Option<i32>,
    #[builder(into)]
    #[serde(rename = "minCount")]
    pub r#min_count: Option<i32>,
    /// The name which should be used for the default Kubernetes Node Pool.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Option<i32>,
    /// A map of Kubernetes labels which should be applied to nodes in the Default Node Pool.
    #[builder(into)]
    #[serde(rename = "nodeLabels")]
    pub r#node_labels: Option<std::collections::HashMap<String, String>>,
    /// A `node_network_profile` block as documented below.
    #[builder(into)]
    #[serde(rename = "nodeNetworkProfile")]
    pub r#node_network_profile: Option<Box<super::super::types::containerservice::KubernetesClusterDefaultNodePoolNodeNetworkProfile>>,
    /// Should nodes in this Node Pool have a Public IP Address? `temporary_name_for_rotation` must be specified when changing this property.
    #[builder(into)]
    #[serde(rename = "nodePublicIpEnabled")]
    pub r#node_public_ip_enabled: Option<bool>,
    /// Resource ID for the Public IP Addresses Prefix for the nodes in this Node Pool. `node_public_ip_enabled` should be `true`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "nodePublicIpPrefixId")]
    pub r#node_public_ip_prefix_id: Option<String>,
    /// Enabling this option will taint default node pool with `CriticalAddonsOnly=true:NoSchedule` taint. `temporary_name_for_rotation` must be specified when changing this property.
    #[builder(into)]
    #[serde(rename = "onlyCriticalAddonsEnabled")]
    pub r#only_critical_addons_enabled: Option<bool>,
    /// Version of Kubernetes used for the Agents. If not specified, the default node pool will be created with the version specified by `kubernetes_version`. If both are unspecified, the latest recommended version will be used at provisioning time (but won't auto-upgrade). AKS does not require an exact patch version to be specified, minor version aliases such as `1.22` are also supported. - The minor version's latest GA patch is automatically chosen in that case. More details can be found in [the documentation](https://docs.microsoft.com/en-us/azure/aks/supported-kubernetes-versions?tabs=azure-cli#alias-minor-version).
    /// 
    /// > **Note:** This version must be supported by the Kubernetes Cluster - as such the version of Kubernetes used on the Cluster/Control Plane may need to be upgraded first.
    #[builder(into)]
    #[serde(rename = "orchestratorVersion")]
    pub r#orchestrator_version: Option<String>,
    /// The size of the OS Disk which should be used for each agent in the Node Pool. `temporary_name_for_rotation` must be specified when attempting a change.
    #[builder(into)]
    #[serde(rename = "osDiskSizeGb")]
    pub r#os_disk_size_gb: Option<i32>,
    /// The type of disk which should be used for the Operating System. Possible values are `Ephemeral` and `Managed`. Defaults to `Managed`. `temporary_name_for_rotation` must be specified when attempting a change.
    #[builder(into)]
    #[serde(rename = "osDiskType")]
    pub r#os_disk_type: Option<String>,
    /// Specifies the OS SKU used by the agent pool. Possible values are `AzureLinux`, `Ubuntu`, `Windows2019` and `Windows2022`. If not specified, the default is `Ubuntu` if OSType=Linux or `Windows2019` if OSType=Windows. And the default Windows OSSKU will be changed to `Windows2022` after Windows2019 is deprecated. Changing this from `AzureLinux` or `Ubuntu` to `AzureLinux` or `Ubuntu` will not replace the resource, otherwise `temporary_name_for_rotation` must be specified when attempting a change.
    #[builder(into)]
    #[serde(rename = "osSku")]
    pub r#os_sku: Option<String>,
    /// The ID of the Subnet where the pods in the default Node Pool should exist.
    #[builder(into)]
    #[serde(rename = "podSubnetId")]
    pub r#pod_subnet_id: Option<String>,
    /// The ID of the Proximity Placement Group. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "proximityPlacementGroupId")]
    pub r#proximity_placement_group_id: Option<String>,
    /// Specifies the autoscaling behaviour of the Kubernetes Cluster. Allowed values are `Delete` and `Deallocate`. Defaults to `Delete`.
    #[builder(into)]
    #[serde(rename = "scaleDownMode")]
    pub r#scale_down_mode: Option<String>,
    /// The ID of the Snapshot which should be used to create this default Node Pool. `temporary_name_for_rotation` must be specified when changing this property.
    #[builder(into)]
    #[serde(rename = "snapshotId")]
    pub r#snapshot_id: Option<String>,
    /// A mapping of tags to assign to the Node Pool.
    /// 
    /// > At this time there's a bug in the AKS API where Tags for a Node Pool are not stored in the correct case - you may wish to use `ignore_changes` functionality to ignore changes to the casing until this is fixed in the AKS API.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// Specifies the name of the temporary node pool used to cycle the default node pool for VM resizing.
    #[builder(into)]
    #[serde(rename = "temporaryNameForRotation")]
    pub r#temporary_name_for_rotation: Option<String>,
    /// The type of Node Pool which should be created. Possible values are `VirtualMachineScaleSets`. Defaults to `VirtualMachineScaleSets`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** When creating a cluster that supports multiple node pools, the cluster must use `VirtualMachineScaleSets`. For more information on the limitations of clusters using multiple node pools see [the documentation](https://learn.microsoft.com/en-us/azure/aks/use-multiple-node-pools#limitations).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// Used to specify whether the UltraSSD is enabled in the Default Node Pool. Defaults to `false`. See [the documentation](https://docs.microsoft.com/azure/aks/use-ultra-disks) for more information. `temporary_name_for_rotation` must be specified when attempting a change.
    #[builder(into)]
    #[serde(rename = "ultraSsdEnabled")]
    pub r#ultra_ssd_enabled: Option<bool>,
    /// A `upgrade_settings` block as documented below.
    #[builder(into)]
    #[serde(rename = "upgradeSettings")]
    pub r#upgrade_settings: Option<Box<super::super::types::containerservice::KubernetesClusterDefaultNodePoolUpgradeSettings>>,
    /// The size of the Virtual Machine, such as `Standard_DS2_v2`. `temporary_name_for_rotation` must be specified when attempting a resize.
    #[builder(into)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: String,
    /// The ID of a Subnet where the Kubernetes Node Pool should exist.
    /// 
    /// > **Note:** A Route Table must be configured on this Subnet.
    #[builder(into)]
    #[serde(rename = "vnetSubnetId")]
    pub r#vnet_subnet_id: Option<String>,
    /// Specifies the workload runtime used by the node pool. Possible value is `OCIContainer`.
    #[builder(into)]
    #[serde(rename = "workloadRuntime")]
    pub r#workload_runtime: Option<String>,
    /// Specifies a list of Availability Zones in which this Kubernetes Cluster should be located. `temporary_name_for_rotation` must be specified when changing this property.
    /// 
    /// > **Note:** This requires that the `type` is set to `VirtualMachineScaleSets` and that `load_balancer_sku` is set to `standard`.
    #[builder(into)]
    #[serde(rename = "zones")]
    pub r#zones: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterDefaultNodePool {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "auto_scaling_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_scaling_enabled,
                )
                .await,
            );
            map.insert(
                "capacity_reservation_group_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#capacity_reservation_group_id,
                )
                .await,
            );
            map.insert(
                "fips_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fips_enabled,
                )
                .await,
            );
            map.insert(
                "gpu_instance".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gpu_instance,
                )
                .await,
            );
            map.insert(
                "host_encryption_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_encryption_enabled,
                )
                .await,
            );
            map.insert(
                "host_group_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_group_id,
                )
                .await,
            );
            map.insert(
                "kubelet_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kubelet_config,
                )
                .await,
            );
            map.insert(
                "kubelet_disk_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kubelet_disk_type,
                )
                .await,
            );
            map.insert(
                "linux_os_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#linux_os_config,
                )
                .await,
            );
            map.insert(
                "max_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_count,
                )
                .await,
            );
            map.insert(
                "max_pods".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_pods,
                )
                .await,
            );
            map.insert(
                "min_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_count,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "node_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_count,
                )
                .await,
            );
            map.insert(
                "node_labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_labels,
                )
                .await,
            );
            map.insert(
                "node_network_profile".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_network_profile,
                )
                .await,
            );
            map.insert(
                "node_public_ip_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_public_ip_enabled,
                )
                .await,
            );
            map.insert(
                "node_public_ip_prefix_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_public_ip_prefix_id,
                )
                .await,
            );
            map.insert(
                "only_critical_addons_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#only_critical_addons_enabled,
                )
                .await,
            );
            map.insert(
                "orchestrator_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#orchestrator_version,
                )
                .await,
            );
            map.insert(
                "os_disk_size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os_disk_size_gb,
                )
                .await,
            );
            map.insert(
                "os_disk_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os_disk_type,
                )
                .await,
            );
            map.insert(
                "os_sku".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os_sku,
                )
                .await,
            );
            map.insert(
                "pod_subnet_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pod_subnet_id,
                )
                .await,
            );
            map.insert(
                "proximity_placement_group_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#proximity_placement_group_id,
                )
                .await,
            );
            map.insert(
                "scale_down_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scale_down_mode,
                )
                .await,
            );
            map.insert(
                "snapshot_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#snapshot_id,
                )
                .await,
            );
            map.insert(
                "tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tags,
                )
                .await,
            );
            map.insert(
                "temporary_name_for_rotation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#temporary_name_for_rotation,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
                )
                .await,
            );
            map.insert(
                "ultra_ssd_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ultra_ssd_enabled,
                )
                .await,
            );
            map.insert(
                "upgrade_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upgrade_settings,
                )
                .await,
            );
            map.insert(
                "vm_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vm_size,
                )
                .await,
            );
            map.insert(
                "vnet_subnet_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vnet_subnet_id,
                )
                .await,
            );
            map.insert(
                "workload_runtime".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workload_runtime,
                )
                .await,
            );
            map.insert(
                "zones".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zones,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
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
                        let field_value = match fields_map.get("auto_scaling_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_scaling_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#capacity_reservation_group_id: {
                        let field_value = match fields_map.get("capacity_reservation_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity_reservation_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fips_enabled: {
                        let field_value = match fields_map.get("fips_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'fips_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gpu_instance: {
                        let field_value = match fields_map.get("gpu_instance") {
                            Some(value) => value,
                            None => bail!("Missing field 'gpu_instance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_encryption_enabled: {
                        let field_value = match fields_map.get("host_encryption_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_encryption_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_group_id: {
                        let field_value = match fields_map.get("host_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kubelet_config: {
                        let field_value = match fields_map.get("kubelet_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'kubelet_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kubelet_disk_type: {
                        let field_value = match fields_map.get("kubelet_disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'kubelet_disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linux_os_config: {
                        let field_value = match fields_map.get("linux_os_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'linux_os_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_count: {
                        let field_value = match fields_map.get("max_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_pods: {
                        let field_value = match fields_map.get("max_pods") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_pods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_count: {
                        let field_value = match fields_map.get("min_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_labels: {
                        let field_value = match fields_map.get("node_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_network_profile: {
                        let field_value = match fields_map.get("node_network_profile") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_network_profile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_public_ip_enabled: {
                        let field_value = match fields_map.get("node_public_ip_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_public_ip_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_public_ip_prefix_id: {
                        let field_value = match fields_map.get("node_public_ip_prefix_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_public_ip_prefix_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#only_critical_addons_enabled: {
                        let field_value = match fields_map.get("only_critical_addons_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'only_critical_addons_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#orchestrator_version: {
                        let field_value = match fields_map.get("orchestrator_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'orchestrator_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_disk_size_gb: {
                        let field_value = match fields_map.get("os_disk_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_disk_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_disk_type: {
                        let field_value = match fields_map.get("os_disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_sku: {
                        let field_value = match fields_map.get("os_sku") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_sku' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_subnet_id: {
                        let field_value = match fields_map.get("pod_subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proximity_placement_group_id: {
                        let field_value = match fields_map.get("proximity_placement_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'proximity_placement_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_down_mode: {
                        let field_value = match fields_map.get("scale_down_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_down_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot_id: {
                        let field_value = match fields_map.get("snapshot_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("temporary_name_for_rotation") {
                            Some(value) => value,
                            None => bail!("Missing field 'temporary_name_for_rotation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ultra_ssd_enabled: {
                        let field_value = match fields_map.get("ultra_ssd_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ultra_ssd_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upgrade_settings: {
                        let field_value = match fields_map.get("upgrade_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'upgrade_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_size: {
                        let field_value = match fields_map.get("vm_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnet_subnet_id: {
                        let field_value = match fields_map.get("vnet_subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnet_subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workload_runtime: {
                        let field_value = match fields_map.get("workload_runtime") {
                            Some(value) => value,
                            None => bail!("Missing field 'workload_runtime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
