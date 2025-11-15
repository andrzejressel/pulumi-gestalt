#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchTemplateBlockDeviceMappingEbs {
    /// Whether the volume should be destroyed on instance termination.
    /// See [Preserving Amazon EBS Volumes on Instance Termination](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/preserving-volumes-on-termination.html) for more information.
    #[builder(into)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Option<String>,
    /// Enables [EBS encryption](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html) on the volume.
    /// Cannot be used with `snapshot_id`.
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Option<String>,
    /// The amount of provisioned [IOPS](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-io-characteristics.html).
    /// This must be set with a `volume_type` of `"io1/io2/gp3"`.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Option<i32>,
    /// The ARN of the AWS Key Management Service (AWS KMS) customer master key (CMK) to use when creating the encrypted volume.
    /// `encrypted` must be set to `true` when this is set.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// The Snapshot ID to mount.
    #[builder(into)]
    #[serde(rename = "snapshotId")]
    pub r#snapshot_id: Option<String>,
    /// The throughput to provision for a `gp3` volume in MiB/s (specified as an integer, e.g., 500), with a maximum of 1,000 MiB/s.
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: Option<i32>,
    /// The size of the volume in gigabytes.
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Option<i32>,
    /// The volume type.
    /// Can be one of `standard`, `gp2`, `gp3`, `io1`, `io2`, `sc1` or `st1`.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Option<String>,
}
