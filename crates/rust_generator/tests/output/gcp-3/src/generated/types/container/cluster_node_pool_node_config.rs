#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodePoolNodeConfig {
    /// Specifies options for controlling
    /// advanced machine features. Structure is documented below.
    #[builder(into)]
    pub r#advanced_machine_features: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigAdvancedMachineFeatures>>,
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool. This should be of the form projects/[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME]. For more information about protecting resources with Cloud KMS Keys please see: <https://cloud.google.com/compute/docs/disks/customer-managed-encryption>
    #[builder(into)]
    pub r#boot_disk_kms_key: Option<String>,
    /// Configuration for Confidential Nodes feature. Structure is documented below.
    #[builder(into)]
    pub r#confidential_nodes: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigConfidentialNodes>>,
    /// Parameters to customize containerd runtime. Structure is documented below.
    #[builder(into)]
    pub r#containerd_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigContainerdConfig>>,
    /// Size of the disk attached to each node, specified
    /// in GB. The smallest allowed disk size is 10GB. Defaults to 100GB.
    #[builder(into)]
    pub r#disk_size_gb: Option<i32>,
    /// Type of the disk attached to each node
    /// (e.g. 'pd-standard', 'pd-balanced' or 'pd-ssd'). If unspecified, the default disk type is 'pd-standard'
    #[builder(into)]
    pub r#disk_type: Option<String>,
    /// List of kubernetes taints applied to each node. Structure is documented above.
    #[builder(into)]
    pub r#effective_taints: Option<Vec<super::super::types::container::ClusterNodePoolNodeConfigEffectiveTaint>>,
    /// Enabling Confidential Storage will create boot disk with confidential mode. It is disabled by default.
    #[builder(into)]
    pub r#enable_confidential_storage: Option<bool>,
    /// Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk. Structure is documented below.
    /// 
    #[builder(into)]
    pub r#ephemeral_storage_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigEphemeralStorageConfig>>,
    /// Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk. Structure is documented below.
    /// 
    #[builder(into)]
    pub r#ephemeral_storage_local_ssd_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigEphemeralStorageLocalSsdConfig>>,
    /// Parameters for the NCCL Fast Socket feature. If unspecified, NCCL Fast Socket will not be enabled on the node pool.
    /// Node Pool must enable gvnic.
    /// GKE version 1.25.2-gke.1700 or later.
    /// Structure is documented below.
    #[builder(into)]
    pub r#fast_socket: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigFastSocket>>,
    /// Parameters for the Google Container Filesystem (GCFS).
    /// If unspecified, GCFS will not be enabled on the node pool. When enabling this feature you must specify `image_type = "COS_CONTAINERD"` and `node_version` from GKE versions 1.19 or later to use it.
    /// For GKE versions 1.19, 1.20, and 1.21, the recommended minimum `node_version` would be 1.19.15-gke.1300, 1.20.11-gke.1300, and 1.21.5-gke.1300 respectively.
    /// A `machine_type` that has more than 16 GiB of memory is also recommended.
    /// GCFS must be enabled in order to use [image streaming](https://cloud.google.com/kubernetes-engine/docs/how-to/image-streaming).
    /// Structure is documented below.
    /// 
    #[builder(into)]
    pub r#gcfs_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigGcfsConfig>>,
    /// List of the type and count of accelerator cards attached to the instance.
    /// Structure documented below.
    #[builder(into)]
    pub r#guest_accelerators: Option<Vec<super::super::types::container::ClusterNodePoolNodeConfigGuestAccelerator>>,
    /// Google Virtual NIC (gVNIC) is a virtual network interface.
    /// Installing the gVNIC driver allows for more efficient traffic transmission across the Google network infrastructure.
    /// gVNIC is an alternative to the virtIO-based ethernet driver. GKE nodes must use a Container-Optimized OS node image.
    /// GKE node version 1.15.11-gke.15 or later
    /// Structure is documented below.
    /// 
    /// 
    #[builder(into)]
    pub r#gvnic: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigGvnic>>,
    /// The maintenance policy for the hosts on which the GKE VMs run on.
    #[builder(into)]
    pub r#host_maintenance_policy: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigHostMaintenancePolicy>>,
    /// The image type to use for this node. Note that changing the image type
    /// will delete and recreate all nodes in the node pool.
    #[builder(into)]
    pub r#image_type: Option<String>,
    /// Kubelet configuration, currently supported attributes can be found [here](https://cloud.google.com/sdk/gcloud/reference/beta/container/node-pools/create#--system-config-from-file).
    /// Structure is documented below.
    /// 
    /// ```sh
    /// kubelet_config {
    /// cpu_manager_policy   = "static"
    /// cpu_cfs_quota        = true
    /// cpu_cfs_quota_period = "100us"
    /// pod_pids_limit       = 1024
    /// }
    /// ```
    #[builder(into)]
    pub r#kubelet_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigKubeletConfig>>,
    /// The Kubernetes labels (key/value pairs) to be applied to each node. The kubernetes.io/ and k8s.io/ prefixes are
    /// reserved by Kubernetes Core components and cannot be specified.
    #[builder(into)]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// Parameters that can be configured on Linux nodes. Structure is documented below.
    #[builder(into)]
    pub r#linux_node_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigLinuxNodeConfig>>,
    /// Parameters for the local NVMe SSDs. Structure is documented below.
    #[builder(into)]
    pub r#local_nvme_ssd_block_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigLocalNvmeSsdBlockConfig>>,
    /// The amount of local SSD disks that will be
    /// attached to each cluster node. Defaults to 0.
    #[builder(into)]
    pub r#local_ssd_count: Option<i32>,
    /// Possible Local SSD encryption modes:
    /// Accepted values are:
    /// * `STANDARD_ENCRYPTION`: The given node will be encrypted using keys managed by Google infrastructure and the keys wll be deleted when the node is deleted.
    /// * `EPHEMERAL_KEY_ENCRYPTION`: The given node will opt-in for using ephemeral key for encrypting Local SSDs. The Local SSDs will not be able to recover data in case of node crash.
    #[builder(into)]
    pub r#local_ssd_encryption_mode: Option<String>,
    /// Parameter for specifying the type of logging agent used in a node pool. This will override any cluster-wide default value. Valid values include DEFAULT and MAX_THROUGHPUT. See [Increasing logging agent throughput](https://cloud.google.com/stackdriver/docs/solutions/gke/managing-logs#throughput) for more information.
    #[builder(into)]
    pub r#logging_variant: Option<String>,
    /// The name of a Google Compute Engine machine type.
    /// Defaults to `e2-medium`. To create a custom machine type, value should be set as specified
    /// [here](https://cloud.google.com/compute/docs/reference/latest/instances#machineType).
    #[builder(into)]
    pub r#machine_type: Option<String>,
    /// The metadata key/value pairs assigned to instances in
    /// the cluster. From GKE `1.12` onwards, `disable-legacy-endpoints` is set to
    /// `true` by the API; if `metadata` is set but that default value is not
    /// included, the provider will attempt to unset the value. To avoid this, set the
    /// value in your config.
    #[builder(into)]
    pub r#metadata: Option<std::collections::HashMap<String, String>>,
    /// Minimum CPU platform to be used by this instance.
    /// The instance may be scheduled on the specified or newer CPU platform. Applicable
    /// values are the friendly names of CPU platforms, such as `Intel Haswell`. See the
    /// [official documentation](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform)
    /// for more information.
    #[builder(into)]
    pub r#min_cpu_platform: Option<String>,
    /// Setting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on [sole tenant nodes](https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes).
    #[builder(into)]
    pub r#node_group: Option<String>,
    /// The set of Google API scopes to be made available
    /// on all of the node VMs under the "default" service account.
    /// Use the "https://www.googleapis.com/auth/cloud-platform" scope to grant access to all APIs. It is recommended that you set `service_account` to a non-default service account and grant IAM roles to that service account for only the resources that it needs.
    /// 
    /// See the [official documentation](https://cloud.google.com/kubernetes-engine/docs/how-to/access-scopes) for information on migrating off of legacy access scopes.
    #[builder(into)]
    pub r#oauth_scopes: Option<Vec<String>>,
    /// A boolean that represents whether or not the underlying node VMs
    /// are preemptible. See the [official documentation](https://cloud.google.com/container-engine/docs/preemptible-vm)
    /// for more information. Defaults to false.
    #[builder(into)]
    pub r#preemptible: Option<bool>,
    /// The configuration of the desired reservation which instances could take capacity from. Structure is documented below.
    #[builder(into)]
    pub r#reservation_affinity: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigReservationAffinity>>,
    /// The GCP labels (key/value pairs) to be applied to each node. Refer [here](https://cloud.google.com/kubernetes-engine/docs/how-to/creating-managing-labels)
    /// for how these labels are applied to clusters, node pools and nodes.
    #[builder(into)]
    pub r#resource_labels: Option<std::collections::HashMap<String, String>>,
    /// A map of resource manager tag keys and values to be attached to the nodes for managing Compute Engine firewalls using Network Firewall Policies. Tags must be according to specifications found [here](https://cloud.google.com/vpc/docs/tags-firewalls-overview#specifications). A maximum of 5 tag key-value pairs can be specified. Existing tags will be replaced with new values. Tags must be in one of the following formats ([KEY]=[VALUE]) 1. `tagKeys/{tag_key_id}=tagValues/{tag_value_id}` 2. `{org_id}/{tag_key_name}={tag_value_name}` 3. `{project_id}/{tag_key_name}={tag_value_name}`.
    #[builder(into)]
    pub r#resource_manager_tags: Option<std::collections::HashMap<String, String>>,
    /// Sandbox configuration for this node.
    #[builder(into)]
    pub r#sandbox_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigSandboxConfig>>,
    /// Parameters for secondary boot disks to preload container images and data on new nodes. Structure is documented below. `gcfs_config` must be `enabled=true` for this feature to work. `min_master_version` must also be set to use GKE 1.28.3-gke.106700 or later versions.
    #[builder(into)]
    pub r#secondary_boot_disks: Option<Vec<super::super::types::container::ClusterNodePoolNodeConfigSecondaryBootDisk>>,
    /// The service account to be used by the Node VMs.
    /// If not specified, the "default" service account is used.
    #[builder(into)]
    pub r#service_account: Option<String>,
    /// Shielded Instance options. Structure is documented below.
    #[builder(into)]
    pub r#shielded_instance_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigShieldedInstanceConfig>>,
    /// Allows specifying multiple [node affinities](https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes#node_affinity_and_anti-affinity) useful for running workloads on [sole tenant nodes](https://cloud.google.com/kubernetes-engine/docs/how-to/sole-tenancy). `node_affinity` structure is documented below.
    /// 
    #[builder(into)]
    pub r#sole_tenant_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigSoleTenantConfig>>,
    /// A boolean that represents whether the underlying node VMs are spot.
    /// See the [official documentation](https://cloud.google.com/kubernetes-engine/docs/concepts/spot-vms)
    /// for more information. Defaults to false.
    #[builder(into)]
    pub r#spot: Option<bool>,
    /// The list of Storage Pools where boot disks are provisioned.
    #[builder(into)]
    pub r#storage_pools: Option<Vec<String>>,
    /// The list of instance tags applied to all nodes. Tags are used to identify
    /// valid sources or targets for network firewalls.
    #[builder(into)]
    pub r#tags: Option<Vec<String>>,
    /// A list of [Kubernetes taints](https://kubernetes.io/docs/concepts/configuration/taint-and-toleration/)
    /// to apply to nodes. GKE's API can only set this field on cluster creation.
    /// However, GKE will add taints to your nodes if you enable certain features such
    /// as GPUs. If this field is set, any diffs on this field will cause the provider to
    /// recreate the underlying resource. Taint values can be updated safely in
    /// Kubernetes (eg. through `kubectl`), and it's recommended that you do not use
    /// this field to manage taints. If you do, `lifecycle.ignore_changes` is
    /// recommended. Structure is documented below.
    #[builder(into)]
    pub r#taints: Option<Vec<super::super::types::container::ClusterNodePoolNodeConfigTaint>>,
    /// Metadata configuration to expose to workloads on the node pool.
    /// Structure is documented below.
    #[builder(into)]
    pub r#workload_metadata_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigWorkloadMetadataConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterNodePoolNodeConfig {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterNodePoolNodeConfig {
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
