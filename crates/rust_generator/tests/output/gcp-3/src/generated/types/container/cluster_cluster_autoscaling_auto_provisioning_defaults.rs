#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterAutoscalingAutoProvisioningDefaults {
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool. This should be of the form projects/[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME]. For more information about protecting resources with Cloud KMS Keys please see: https://cloud.google.com/compute/docs/disks/customer-managed-encryption
    #[builder(into)]
    #[serde(rename = "bootDiskKmsKey")]
    pub r#boot_disk_kms_key: Option<String>,
    /// Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB. Defaults to `100`
    #[builder(into)]
    #[serde(rename = "diskSize")]
    pub r#disk_size: Option<i32>,
    /// Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd', 'pd-balanced', or 'hyperdisk-balanced'). Defaults to `hyperdisk-balanced` if `hyperdisk-balanced` is supported and `pd-balanced` is not supported for the machine type; otherwise defaults to `pd-balanced`.
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Option<String>,
    /// The default image type used by NAP once a new node pool is being created. Please note that according to the [official documentation](https://cloud.google.com/kubernetes-engine/docs/how-to/node-auto-provisioning#default-image-type) the value must be one of the [COS_CONTAINERD, COS, UBUNTU_CONTAINERD, UBUNTU]. __NOTE__ : COS AND UBUNTU are deprecated as of `GKE 1.24`
    #[builder(into)]
    #[serde(rename = "imageType")]
    pub r#image_type: Option<String>,
    /// NodeManagement configuration for this NodePool. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "management")]
    pub r#management: Option<Box<super::super::types::container::ClusterClusterAutoscalingAutoProvisioningDefaultsManagement>>,
    /// Minimum CPU platform to be used for NAP created node pools. The instance may be scheduled on the
    /// specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such
    /// as "Intel Haswell" or "Intel Sandy Bridge".
    #[builder(into)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Option<String>,
    /// Scopes that are used by NAP and GKE Autopilot when creating node pools. Use the "https://www.googleapis.com/auth/cloud-platform" scope to grant access to all APIs. It is recommended that you set `service_account` to a non-default service account and grant IAM roles to that service account for only the resources that it needs.
    /// 
    /// > `monitoring.write` is always enabled regardless of user input.  `monitoring` and `logging.write` may also be enabled depending on the values for `monitoring_service` and `logging_service`.
    #[builder(into)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Option<Vec<String>>,
    /// The Google Cloud Platform Service Account to be used by the node VMs created by GKE Autopilot or NAP.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Option<String>,
    /// Shielded Instance options. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Option<Box<super::super::types::container::ClusterClusterAutoscalingAutoProvisioningDefaultsShieldedInstanceConfig>>,
    /// Specifies the upgrade settings for NAP created node pools
    #[builder(into)]
    #[serde(rename = "upgradeSettings")]
    pub r#upgrade_settings: Option<Box<super::super::types::container::ClusterClusterAutoscalingAutoProvisioningDefaultsUpgradeSettings>>,
}
