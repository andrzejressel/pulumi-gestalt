#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NodePoolNodeConfig {
    /// Specifies options for controlling advanced machine features.
    #[builder(into)]
    pub r#advanced_machine_features: Option<Box<super::super::types::container::NodePoolNodeConfigAdvancedMachineFeatures>>,
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool.
    #[builder(into)]
    pub r#boot_disk_kms_key: Option<String>,
    /// Configuration for the confidential nodes feature, which makes nodes run on confidential VMs. Warning: This configuration can't be changed (or added/removed) after pool creation without deleting and recreating the entire pool.
    #[builder(into)]
    pub r#confidential_nodes: Option<Box<super::super::types::container::NodePoolNodeConfigConfidentialNodes>>,
    /// Parameters for containerd configuration.
    #[builder(into)]
    pub r#containerd_config: Option<Box<super::super::types::container::NodePoolNodeConfigContainerdConfig>>,
    /// Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB.
    #[builder(into)]
    pub r#disk_size_gb: Option<i32>,
    /// Type of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd
    #[builder(into)]
    pub r#disk_type: Option<String>,
    /// List of kubernetes taints applied to each node.
    #[builder(into)]
    pub r#effective_taints: Option<Vec<super::super::types::container::NodePoolNodeConfigEffectiveTaint>>,
    /// If enabled boot disks are configured with confidential mode.
    #[builder(into)]
    pub r#enable_confidential_storage: Option<bool>,
    /// Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk.
    #[builder(into)]
    pub r#ephemeral_storage_config: Option<Box<super::super::types::container::NodePoolNodeConfigEphemeralStorageConfig>>,
    /// Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk.
    #[builder(into)]
    pub r#ephemeral_storage_local_ssd_config: Option<Box<super::super::types::container::NodePoolNodeConfigEphemeralStorageLocalSsdConfig>>,
    /// Enable or disable NCCL Fast Socket in the node pool.
    #[builder(into)]
    pub r#fast_socket: Option<Box<super::super::types::container::NodePoolNodeConfigFastSocket>>,
    /// GCFS configuration for this node.
    #[builder(into)]
    pub r#gcfs_config: Option<Box<super::super::types::container::NodePoolNodeConfigGcfsConfig>>,
    /// List of the type and count of accelerator cards attached to the instance.
    #[builder(into)]
    pub r#guest_accelerators: Option<Vec<super::super::types::container::NodePoolNodeConfigGuestAccelerator>>,
    /// Enable or disable gvnic in the node pool.
    #[builder(into)]
    pub r#gvnic: Option<Box<super::super::types::container::NodePoolNodeConfigGvnic>>,
    /// The maintenance policy for the hosts on which the GKE VMs run on.
    #[builder(into)]
    pub r#host_maintenance_policy: Option<Box<super::super::types::container::NodePoolNodeConfigHostMaintenancePolicy>>,
    /// The image type to use for this node. Note that for a given image type, the latest version of it will be used.
    #[builder(into)]
    pub r#image_type: Option<String>,
    /// Node kubelet configs.
    #[builder(into)]
    pub r#kubelet_config: Option<Box<super::super::types::container::NodePoolNodeConfigKubeletConfig>>,
    /// The map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node.
    #[builder(into)]
    pub r#labels: Option<std::collections::BTreeMap<String, String>>,
    /// Parameters that can be configured on Linux nodes.
    #[builder(into)]
    pub r#linux_node_config: Option<Box<super::super::types::container::NodePoolNodeConfigLinuxNodeConfig>>,
    /// Parameters for raw-block local NVMe SSDs.
    #[builder(into)]
    pub r#local_nvme_ssd_block_config: Option<Box<super::super::types::container::NodePoolNodeConfigLocalNvmeSsdBlockConfig>>,
    /// The number of local SSD disks to be attached to the node.
    #[builder(into)]
    pub r#local_ssd_count: Option<i32>,
    /// LocalSsdEncryptionMode specified the method used for encrypting the local SSDs attached to the node.
    #[builder(into)]
    pub r#local_ssd_encryption_mode: Option<String>,
    /// Type of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT.
    #[builder(into)]
    pub r#logging_variant: Option<String>,
    /// The name of a Google Compute Engine machine type.
    #[builder(into)]
    pub r#machine_type: Option<String>,
    /// The metadata key/value pairs assigned to instances in the cluster.
    #[builder(into)]
    pub r#metadata: Option<std::collections::BTreeMap<String, String>>,
    /// Minimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform.
    #[builder(into)]
    pub r#min_cpu_platform: Option<String>,
    /// Setting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on sole tenant nodes.
    #[builder(into)]
    pub r#node_group: Option<String>,
    /// The set of Google API scopes to be made available on all of the node VMs.
    #[builder(into)]
    pub r#oauth_scopes: Option<Vec<String>>,
    /// Whether the nodes are created as preemptible VM instances.
    #[builder(into)]
    pub r#preemptible: Option<bool>,
    /// The configuration of the desired reservation which instances could take capacity from.
    /// Structure is documented below.
    /// 
    /// <a name="nested_autoscaling"></a>The `autoscaling` block supports (either total or per zone limits are required):
    #[builder(into)]
    pub r#reservation_affinity: Option<Box<super::super::types::container::NodePoolNodeConfigReservationAffinity>>,
    /// The GCE resource labels (a map of key/value pairs) to be applied to the node pool.
    #[builder(into)]
    pub r#resource_labels: Option<std::collections::BTreeMap<String, String>>,
    /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty.
    #[builder(into)]
    pub r#resource_manager_tags: Option<std::collections::BTreeMap<String, String>>,
    /// Sandbox configuration for this node.
    #[builder(into)]
    pub r#sandbox_config: Option<Box<super::super::types::container::NodePoolNodeConfigSandboxConfig>>,
    /// Secondary boot disks for preloading data or container images.
    #[builder(into)]
    pub r#secondary_boot_disks: Option<Vec<super::super::types::container::NodePoolNodeConfigSecondaryBootDisk>>,
    /// The Google Cloud Platform Service Account to be used by the node VMs.
    #[builder(into)]
    pub r#service_account: Option<String>,
    /// Shielded Instance options.
    #[builder(into)]
    pub r#shielded_instance_config: Option<Box<super::super::types::container::NodePoolNodeConfigShieldedInstanceConfig>>,
    /// Node affinity options for sole tenant node pools.
    #[builder(into)]
    pub r#sole_tenant_config: Option<Box<super::super::types::container::NodePoolNodeConfigSoleTenantConfig>>,
    /// Whether the nodes are created as spot VM instances.
    #[builder(into)]
    pub r#spot: Option<bool>,
    /// The list of Storage Pools where boot disks are provisioned.
    #[builder(into)]
    pub r#storage_pools: Option<Vec<String>>,
    /// The list of instance tags applied to all nodes.
    #[builder(into)]
    pub r#tags: Option<Vec<String>>,
    /// List of Kubernetes taints to be applied to each node.
    #[builder(into)]
    pub r#taints: Option<Vec<super::super::types::container::NodePoolNodeConfigTaint>>,
    /// The workload metadata configuration for this node.
    #[builder(into)]
    pub r#workload_metadata_config: Option<Box<super::super::types::container::NodePoolNodeConfigWorkloadMetadataConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NodePoolNodeConfig {
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
                    "advancedMachineFeatures",
                    &self.r#advanced_machine_features,
                ),
                to_pulumi_object_field(
                    "bootDiskKmsKey",
                    &self.r#boot_disk_kms_key,
                ),
                to_pulumi_object_field(
                    "confidentialNodes",
                    &self.r#confidential_nodes,
                ),
                to_pulumi_object_field(
                    "containerdConfig",
                    &self.r#containerd_config,
                ),
                to_pulumi_object_field(
                    "diskSizeGb",
                    &self.r#disk_size_gb,
                ),
                to_pulumi_object_field(
                    "diskType",
                    &self.r#disk_type,
                ),
                to_pulumi_object_field(
                    "effectiveTaints",
                    &self.r#effective_taints,
                ),
                to_pulumi_object_field(
                    "enableConfidentialStorage",
                    &self.r#enable_confidential_storage,
                ),
                to_pulumi_object_field(
                    "ephemeralStorageConfig",
                    &self.r#ephemeral_storage_config,
                ),
                to_pulumi_object_field(
                    "ephemeralStorageLocalSsdConfig",
                    &self.r#ephemeral_storage_local_ssd_config,
                ),
                to_pulumi_object_field(
                    "fastSocket",
                    &self.r#fast_socket,
                ),
                to_pulumi_object_field(
                    "gcfsConfig",
                    &self.r#gcfs_config,
                ),
                to_pulumi_object_field(
                    "guestAccelerators",
                    &self.r#guest_accelerators,
                ),
                to_pulumi_object_field(
                    "gvnic",
                    &self.r#gvnic,
                ),
                to_pulumi_object_field(
                    "hostMaintenancePolicy",
                    &self.r#host_maintenance_policy,
                ),
                to_pulumi_object_field(
                    "imageType",
                    &self.r#image_type,
                ),
                to_pulumi_object_field(
                    "kubeletConfig",
                    &self.r#kubelet_config,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "linuxNodeConfig",
                    &self.r#linux_node_config,
                ),
                to_pulumi_object_field(
                    "localNvmeSsdBlockConfig",
                    &self.r#local_nvme_ssd_block_config,
                ),
                to_pulumi_object_field(
                    "localSsdCount",
                    &self.r#local_ssd_count,
                ),
                to_pulumi_object_field(
                    "localSsdEncryptionMode",
                    &self.r#local_ssd_encryption_mode,
                ),
                to_pulumi_object_field(
                    "loggingVariant",
                    &self.r#logging_variant,
                ),
                to_pulumi_object_field(
                    "machineType",
                    &self.r#machine_type,
                ),
                to_pulumi_object_field(
                    "metadata",
                    &self.r#metadata,
                ),
                to_pulumi_object_field(
                    "minCpuPlatform",
                    &self.r#min_cpu_platform,
                ),
                to_pulumi_object_field(
                    "nodeGroup",
                    &self.r#node_group,
                ),
                to_pulumi_object_field(
                    "oauthScopes",
                    &self.r#oauth_scopes,
                ),
                to_pulumi_object_field(
                    "preemptible",
                    &self.r#preemptible,
                ),
                to_pulumi_object_field(
                    "reservationAffinity",
                    &self.r#reservation_affinity,
                ),
                to_pulumi_object_field(
                    "resourceLabels",
                    &self.r#resource_labels,
                ),
                to_pulumi_object_field(
                    "resourceManagerTags",
                    &self.r#resource_manager_tags,
                ),
                to_pulumi_object_field(
                    "sandboxConfig",
                    &self.r#sandbox_config,
                ),
                to_pulumi_object_field(
                    "secondaryBootDisks",
                    &self.r#secondary_boot_disks,
                ),
                to_pulumi_object_field(
                    "serviceAccount",
                    &self.r#service_account,
                ),
                to_pulumi_object_field(
                    "shieldedInstanceConfig",
                    &self.r#shielded_instance_config,
                ),
                to_pulumi_object_field(
                    "soleTenantConfig",
                    &self.r#sole_tenant_config,
                ),
                to_pulumi_object_field(
                    "spot",
                    &self.r#spot,
                ),
                to_pulumi_object_field(
                    "storagePools",
                    &self.r#storage_pools,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "taints",
                    &self.r#taints,
                ),
                to_pulumi_object_field(
                    "workloadMetadataConfig",
                    &self.r#workload_metadata_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NodePoolNodeConfig {
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
                        let field_value = match fields_map.get("advancedMachineFeatures") {
                            Some(value) => value,
                            None => bail!("Missing field 'advancedMachineFeatures' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#boot_disk_kms_key: {
                        let field_value = match fields_map.get("bootDiskKmsKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'bootDiskKmsKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#confidential_nodes: {
                        let field_value = match fields_map.get("confidentialNodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'confidentialNodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#containerd_config: {
                        let field_value = match fields_map.get("containerdConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerdConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_size_gb: {
                        let field_value = match fields_map.get("diskSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_type: {
                        let field_value = match fields_map.get("diskType") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#effective_taints: {
                        let field_value = match fields_map.get("effectiveTaints") {
                            Some(value) => value,
                            None => bail!("Missing field 'effectiveTaints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_confidential_storage: {
                        let field_value = match fields_map.get("enableConfidentialStorage") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableConfidentialStorage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ephemeral_storage_config: {
                        let field_value = match fields_map.get("ephemeralStorageConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'ephemeralStorageConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ephemeral_storage_local_ssd_config: {
                        let field_value = match fields_map.get("ephemeralStorageLocalSsdConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'ephemeralStorageLocalSsdConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fast_socket: {
                        let field_value = match fields_map.get("fastSocket") {
                            Some(value) => value,
                            None => bail!("Missing field 'fastSocket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcfs_config: {
                        let field_value = match fields_map.get("gcfsConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcfsConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#guest_accelerators: {
                        let field_value = match fields_map.get("guestAccelerators") {
                            Some(value) => value,
                            None => bail!("Missing field 'guestAccelerators' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gvnic: {
                        let field_value = match fields_map.get("gvnic") {
                            Some(value) => value,
                            None => bail!("Missing field 'gvnic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_maintenance_policy: {
                        let field_value = match fields_map.get("hostMaintenancePolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostMaintenancePolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_type: {
                        let field_value = match fields_map.get("imageType") {
                            Some(value) => value,
                            None => bail!("Missing field 'imageType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linux_node_config: {
                        let field_value = match fields_map.get("linuxNodeConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'linuxNodeConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_nvme_ssd_block_config: {
                        let field_value = match fields_map.get("localNvmeSsdBlockConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'localNvmeSsdBlockConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_ssd_count: {
                        let field_value = match fields_map.get("localSsdCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'localSsdCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_ssd_encryption_mode: {
                        let field_value = match fields_map.get("localSsdEncryptionMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'localSsdEncryptionMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logging_variant: {
                        let field_value = match fields_map.get("loggingVariant") {
                            Some(value) => value,
                            None => bail!("Missing field 'loggingVariant' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_type: {
                        let field_value = match fields_map.get("machineType") {
                            Some(value) => value,
                            None => bail!("Missing field 'machineType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("minCpuPlatform") {
                            Some(value) => value,
                            None => bail!("Missing field 'minCpuPlatform' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_group: {
                        let field_value = match fields_map.get("nodeGroup") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeGroup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_scopes: {
                        let field_value = match fields_map.get("oauthScopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauthScopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#reservation_affinity: {
                        let field_value = match fields_map.get("reservationAffinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'reservationAffinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_labels: {
                        let field_value = match fields_map.get("resourceLabels") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceLabels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_manager_tags: {
                        let field_value = match fields_map.get("resourceManagerTags") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceManagerTags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sandbox_config: {
                        let field_value = match fields_map.get("sandboxConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'sandboxConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_boot_disks: {
                        let field_value = match fields_map.get("secondaryBootDisks") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondaryBootDisks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account: {
                        let field_value = match fields_map.get("serviceAccount") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shielded_instance_config: {
                        let field_value = match fields_map.get("shieldedInstanceConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'shieldedInstanceConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sole_tenant_config: {
                        let field_value = match fields_map.get("soleTenantConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'soleTenantConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("storagePools") {
                            Some(value) => value,
                            None => bail!("Missing field 'storagePools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#workload_metadata_config: {
                        let field_value = match fields_map.get("workloadMetadataConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'workloadMetadataConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
