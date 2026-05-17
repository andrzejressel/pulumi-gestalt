#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodeConfig {
    /// Specifies options for controlling advanced machine features.
    #[builder(into)]
    #[serde(rename = "advancedMachineFeatures")]
    pub r#advanced_machine_features: Vec<super::super::types::container::GetClusterNodeConfigAdvancedMachineFeature>,
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool.
    #[builder(into)]
    #[serde(rename = "bootDiskKmsKey")]
    pub r#boot_disk_kms_key: String,
    /// Configuration for the confidential nodes feature, which makes nodes run on confidential VMs. Warning: This configuration can't be changed (or added/removed) after pool creation without deleting and recreating the entire pool.
    #[builder(into)]
    #[serde(rename = "confidentialNodes")]
    pub r#confidential_nodes: Vec<super::super::types::container::GetClusterNodeConfigConfidentialNode>,
    /// Parameters for containerd configuration.
    #[builder(into)]
    #[serde(rename = "containerdConfigs")]
    pub r#containerd_configs: Vec<super::super::types::container::GetClusterNodeConfigContainerdConfig>,
    /// Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: i32,
    /// Type of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: String,
    /// List of kubernetes taints applied to each node.
    #[builder(into)]
    #[serde(rename = "effectiveTaints")]
    pub r#effective_taints: Vec<super::super::types::container::GetClusterNodeConfigEffectiveTaint>,
    /// If enabled boot disks are configured with confidential mode.
    #[builder(into)]
    #[serde(rename = "enableConfidentialStorage")]
    pub r#enable_confidential_storage: bool,
    /// Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk.
    #[builder(into)]
    #[serde(rename = "ephemeralStorageConfigs")]
    pub r#ephemeral_storage_configs: Vec<super::super::types::container::GetClusterNodeConfigEphemeralStorageConfig>,
    /// Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk.
    #[builder(into)]
    #[serde(rename = "ephemeralStorageLocalSsdConfigs")]
    pub r#ephemeral_storage_local_ssd_configs: Vec<super::super::types::container::GetClusterNodeConfigEphemeralStorageLocalSsdConfig>,
    /// Enable or disable NCCL Fast Socket in the node pool.
    #[builder(into)]
    #[serde(rename = "fastSockets")]
    pub r#fast_sockets: Vec<super::super::types::container::GetClusterNodeConfigFastSocket>,
    /// GCFS configuration for this node.
    #[builder(into)]
    #[serde(rename = "gcfsConfigs")]
    pub r#gcfs_configs: Vec<super::super::types::container::GetClusterNodeConfigGcfsConfig>,
    /// List of the type and count of accelerator cards attached to the instance.
    #[builder(into)]
    #[serde(rename = "guestAccelerators")]
    pub r#guest_accelerators: Vec<super::super::types::container::GetClusterNodeConfigGuestAccelerator>,
    /// Enable or disable gvnic in the node pool.
    #[builder(into)]
    #[serde(rename = "gvnics")]
    pub r#gvnics: Vec<super::super::types::container::GetClusterNodeConfigGvnic>,
    /// The maintenance policy for the hosts on which the GKE VMs run on.
    #[builder(into)]
    #[serde(rename = "hostMaintenancePolicies")]
    pub r#host_maintenance_policies: Vec<super::super::types::container::GetClusterNodeConfigHostMaintenancePolicy>,
    /// The image type to use for this node. Note that for a given image type, the latest version of it will be used.
    #[builder(into)]
    #[serde(rename = "imageType")]
    pub r#image_type: String,
    /// Node kubelet configs.
    #[builder(into)]
    #[serde(rename = "kubeletConfigs")]
    pub r#kubelet_configs: Vec<super::super::types::container::GetClusterNodeConfigKubeletConfig>,
    /// The map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// Parameters that can be configured on Linux nodes.
    #[builder(into)]
    #[serde(rename = "linuxNodeConfigs")]
    pub r#linux_node_configs: Vec<super::super::types::container::GetClusterNodeConfigLinuxNodeConfig>,
    /// Parameters for raw-block local NVMe SSDs.
    #[builder(into)]
    #[serde(rename = "localNvmeSsdBlockConfigs")]
    pub r#local_nvme_ssd_block_configs: Vec<super::super::types::container::GetClusterNodeConfigLocalNvmeSsdBlockConfig>,
    /// The number of local SSD disks to be attached to the node.
    #[builder(into)]
    #[serde(rename = "localSsdCount")]
    pub r#local_ssd_count: i32,
    /// LocalSsdEncryptionMode specified the method used for encrypting the local SSDs attached to the node.
    #[builder(into)]
    #[serde(rename = "localSsdEncryptionMode")]
    pub r#local_ssd_encryption_mode: String,
    /// Type of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT.
    #[builder(into)]
    #[serde(rename = "loggingVariant")]
    pub r#logging_variant: String,
    /// The name of a Google Compute Engine machine type.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: String,
    /// The metadata key/value pairs assigned to instances in the cluster.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: std::collections::HashMap<String, String>,
    /// Minimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform.
    #[builder(into)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: String,
    /// Setting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on sole tenant nodes.
    #[builder(into)]
    #[serde(rename = "nodeGroup")]
    pub r#node_group: String,
    /// The set of Google API scopes to be made available on all of the node VMs.
    #[builder(into)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Vec<String>,
    /// Whether the nodes are created as preemptible VM instances.
    #[builder(into)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: bool,
    /// The reservation affinity configuration for the node pool.
    #[builder(into)]
    #[serde(rename = "reservationAffinities")]
    pub r#reservation_affinities: Vec<super::super::types::container::GetClusterNodeConfigReservationAffinity>,
    /// The GCE resource labels (a map of key/value pairs) to be applied to the node pool.
    #[builder(into)]
    #[serde(rename = "resourceLabels")]
    pub r#resource_labels: std::collections::HashMap<String, String>,
    /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty.
    #[builder(into)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: std::collections::HashMap<String, String>,
    /// Sandbox configuration for this node.
    #[builder(into)]
    #[serde(rename = "sandboxConfigs")]
    pub r#sandbox_configs: Vec<super::super::types::container::GetClusterNodeConfigSandboxConfig>,
    /// Secondary boot disks for preloading data or container images.
    #[builder(into)]
    #[serde(rename = "secondaryBootDisks")]
    pub r#secondary_boot_disks: Vec<super::super::types::container::GetClusterNodeConfigSecondaryBootDisk>,
    /// The Google Cloud Platform Service Account to be used by the node VMs.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: String,
    /// Shielded Instance options.
    #[builder(into)]
    #[serde(rename = "shieldedInstanceConfigs")]
    pub r#shielded_instance_configs: Vec<super::super::types::container::GetClusterNodeConfigShieldedInstanceConfig>,
    /// Node affinity options for sole tenant node pools.
    #[builder(into)]
    #[serde(rename = "soleTenantConfigs")]
    pub r#sole_tenant_configs: Vec<super::super::types::container::GetClusterNodeConfigSoleTenantConfig>,
    /// Whether the nodes are created as spot VM instances.
    #[builder(into)]
    #[serde(rename = "spot")]
    pub r#spot: bool,
    /// The list of Storage Pools where boot disks are provisioned.
    #[builder(into)]
    #[serde(rename = "storagePools")]
    pub r#storage_pools: Vec<String>,
    /// The list of instance tags applied to all nodes.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Vec<String>,
    /// List of Kubernetes taints to be applied to each node.
    #[builder(into)]
    #[serde(rename = "taints")]
    pub r#taints: Vec<super::super::types::container::GetClusterNodeConfigTaint>,
    /// The workload metadata configuration for this node.
    #[builder(into)]
    #[serde(rename = "workloadMetadataConfigs")]
    pub r#workload_metadata_configs: Vec<super::super::types::container::GetClusterNodeConfigWorkloadMetadataConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterNodeConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "advanced_machine_features".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#advanced_machine_features,
                )
                .await,
            );
            map.insert(
                "boot_disk_kms_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#boot_disk_kms_key,
                )
                .await,
            );
            map.insert(
                "confidential_nodes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#confidential_nodes,
                )
                .await,
            );
            map.insert(
                "containerd_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#containerd_configs,
                )
                .await,
            );
            map.insert(
                "disk_size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disk_size_gb,
                )
                .await,
            );
            map.insert(
                "disk_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disk_type,
                )
                .await,
            );
            map.insert(
                "effective_taints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#effective_taints,
                )
                .await,
            );
            map.insert(
                "enable_confidential_storage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_confidential_storage,
                )
                .await,
            );
            map.insert(
                "ephemeral_storage_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ephemeral_storage_configs,
                )
                .await,
            );
            map.insert(
                "ephemeral_storage_local_ssd_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ephemeral_storage_local_ssd_configs,
                )
                .await,
            );
            map.insert(
                "fast_sockets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fast_sockets,
                )
                .await,
            );
            map.insert(
                "gcfs_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gcfs_configs,
                )
                .await,
            );
            map.insert(
                "guest_accelerators".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#guest_accelerators,
                )
                .await,
            );
            map.insert(
                "gvnics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gvnics,
                )
                .await,
            );
            map.insert(
                "host_maintenance_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_maintenance_policies,
                )
                .await,
            );
            map.insert(
                "image_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_type,
                )
                .await,
            );
            map.insert(
                "kubelet_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kubelet_configs,
                )
                .await,
            );
            map.insert(
                "labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#labels,
                )
                .await,
            );
            map.insert(
                "linux_node_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#linux_node_configs,
                )
                .await,
            );
            map.insert(
                "local_nvme_ssd_block_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_nvme_ssd_block_configs,
                )
                .await,
            );
            map.insert(
                "local_ssd_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_ssd_count,
                )
                .await,
            );
            map.insert(
                "local_ssd_encryption_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_ssd_encryption_mode,
                )
                .await,
            );
            map.insert(
                "logging_variant".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#logging_variant,
                )
                .await,
            );
            map.insert(
                "machine_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#machine_type,
                )
                .await,
            );
            map.insert(
                "metadata".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metadata,
                )
                .await,
            );
            map.insert(
                "min_cpu_platform".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_cpu_platform,
                )
                .await,
            );
            map.insert(
                "node_group".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_group,
                )
                .await,
            );
            map.insert(
                "oauth_scopes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oauth_scopes,
                )
                .await,
            );
            map.insert(
                "preemptible".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preemptible,
                )
                .await,
            );
            map.insert(
                "reservation_affinities".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reservation_affinities,
                )
                .await,
            );
            map.insert(
                "resource_labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_labels,
                )
                .await,
            );
            map.insert(
                "resource_manager_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_manager_tags,
                )
                .await,
            );
            map.insert(
                "sandbox_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sandbox_configs,
                )
                .await,
            );
            map.insert(
                "secondary_boot_disks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secondary_boot_disks,
                )
                .await,
            );
            map.insert(
                "service_account".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account,
                )
                .await,
            );
            map.insert(
                "shielded_instance_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#shielded_instance_configs,
                )
                .await,
            );
            map.insert(
                "sole_tenant_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sole_tenant_configs,
                )
                .await,
            );
            map.insert(
                "spot".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spot,
                )
                .await,
            );
            map.insert(
                "storage_pools".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_pools,
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
                "taints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#taints,
                )
                .await,
            );
            map.insert(
                "workload_metadata_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workload_metadata_configs,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterNodeConfig {
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
                    r#advanced_machine_features: {
                        let field_value = match fields_map.get("advanced_machine_features") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_machine_features' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#boot_disk_kms_key: {
                        let field_value = match fields_map.get("boot_disk_kms_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'boot_disk_kms_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#confidential_nodes: {
                        let field_value = match fields_map.get("confidential_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'confidential_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#containerd_configs: {
                        let field_value = match fields_map.get("containerd_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerd_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_size_gb: {
                        let field_value = match fields_map.get("disk_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_type: {
                        let field_value = match fields_map.get("disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#effective_taints: {
                        let field_value = match fields_map.get("effective_taints") {
                            Some(value) => value,
                            None => bail!("Missing field 'effective_taints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_confidential_storage: {
                        let field_value = match fields_map.get("enable_confidential_storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_confidential_storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ephemeral_storage_configs: {
                        let field_value = match fields_map.get("ephemeral_storage_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ephemeral_storage_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ephemeral_storage_local_ssd_configs: {
                        let field_value = match fields_map.get("ephemeral_storage_local_ssd_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ephemeral_storage_local_ssd_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fast_sockets: {
                        let field_value = match fields_map.get("fast_sockets") {
                            Some(value) => value,
                            None => bail!("Missing field 'fast_sockets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcfs_configs: {
                        let field_value = match fields_map.get("gcfs_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcfs_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#guest_accelerators: {
                        let field_value = match fields_map.get("guest_accelerators") {
                            Some(value) => value,
                            None => bail!("Missing field 'guest_accelerators' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gvnics: {
                        let field_value = match fields_map.get("gvnics") {
                            Some(value) => value,
                            None => bail!("Missing field 'gvnics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_maintenance_policies: {
                        let field_value = match fields_map.get("host_maintenance_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_maintenance_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_type: {
                        let field_value = match fields_map.get("image_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kubelet_configs: {
                        let field_value = match fields_map.get("kubelet_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'kubelet_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linux_node_configs: {
                        let field_value = match fields_map.get("linux_node_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'linux_node_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_nvme_ssd_block_configs: {
                        let field_value = match fields_map.get("local_nvme_ssd_block_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_nvme_ssd_block_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_ssd_count: {
                        let field_value = match fields_map.get("local_ssd_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_ssd_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_ssd_encryption_mode: {
                        let field_value = match fields_map.get("local_ssd_encryption_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_ssd_encryption_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logging_variant: {
                        let field_value = match fields_map.get("logging_variant") {
                            Some(value) => value,
                            None => bail!("Missing field 'logging_variant' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_type: {
                        let field_value = match fields_map.get("machine_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata: {
                        let field_value = match fields_map.get("metadata") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_cpu_platform: {
                        let field_value = match fields_map.get("min_cpu_platform") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_cpu_platform' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_group: {
                        let field_value = match fields_map.get("node_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_scopes: {
                        let field_value = match fields_map.get("oauth_scopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_scopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preemptible: {
                        let field_value = match fields_map.get("preemptible") {
                            Some(value) => value,
                            None => bail!("Missing field 'preemptible' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reservation_affinities: {
                        let field_value = match fields_map.get("reservation_affinities") {
                            Some(value) => value,
                            None => bail!("Missing field 'reservation_affinities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_labels: {
                        let field_value = match fields_map.get("resource_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_manager_tags: {
                        let field_value = match fields_map.get("resource_manager_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_manager_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sandbox_configs: {
                        let field_value = match fields_map.get("sandbox_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'sandbox_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_boot_disks: {
                        let field_value = match fields_map.get("secondary_boot_disks") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_boot_disks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account: {
                        let field_value = match fields_map.get("service_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shielded_instance_configs: {
                        let field_value = match fields_map.get("shielded_instance_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'shielded_instance_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sole_tenant_configs: {
                        let field_value = match fields_map.get("sole_tenant_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'sole_tenant_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot: {
                        let field_value = match fields_map.get("spot") {
                            Some(value) => value,
                            None => bail!("Missing field 'spot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_pools: {
                        let field_value = match fields_map.get("storage_pools") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_pools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#taints: {
                        let field_value = match fields_map.get("taints") {
                            Some(value) => value,
                            None => bail!("Missing field 'taints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workload_metadata_configs: {
                        let field_value = match fields_map.get("workload_metadata_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'workload_metadata_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
