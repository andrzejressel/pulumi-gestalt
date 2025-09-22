#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceVolumeConfigurationManagedEbsVolume {
    /// Whether the volume should be encrypted. Default value is `true`.
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Option<bool>,
    /// Linux filesystem type for the volume. For volumes created from a snapshot, same filesystem type must be specified that the volume was using when the snapshot was created. Valid values are `ext3`, `ext4`, `xfs`. Default value is `xfs`.
    #[builder(into)]
    #[serde(rename = "fileSystemType")]
    pub r#file_system_type: Option<String>,
    /// Number of I/O operations per second (IOPS).
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Option<i32>,
    /// Amazon Resource Name (ARN) identifier of the Amazon Web Services Key Management Service key to use for Amazon EBS encryption.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Amazon ECS infrastructure IAM role that is used to manage your Amazon Web Services infrastructure. Recommended using the Amazon ECS-managed `AmazonECSInfrastructureRolePolicyForVolumes` IAM policy with this role.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// Size of the volume in GiB. You must specify either a `size_in_gb` or a `snapshot_id`. You can optionally specify a volume size greater than or equal to the snapshot size.
    #[builder(into)]
    #[serde(rename = "sizeInGb")]
    pub r#size_in_gb: Option<i32>,
    /// Snapshot that Amazon ECS uses to create the volume. You must specify either a `size_in_gb` or a `snapshot_id`.
    #[builder(into)]
    #[serde(rename = "snapshotId")]
    pub r#snapshot_id: Option<String>,
    /// The tags to apply to the volume. See below.
    #[builder(into)]
    #[serde(rename = "tagSpecifications")]
    pub r#tag_specifications: Option<Vec<super::super::types::ecs::ServiceVolumeConfigurationManagedEbsVolumeTagSpecification>>,
    /// Throughput to provision for a volume, in MiB/s, with a maximum of 1,000 MiB/s.
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: Option<i32>,
    /// Volume type.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Option<String>,
}
