#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ImageRecipeBlockDeviceMappingEbs {
    /// Whether to delete the volume on termination. Defaults to unset, which is the value inherited from the parent image.
    #[builder(into)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Option<String>,
    /// Whether to encrypt the volume. Defaults to unset, which is the value inherited from the parent image.
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Option<String>,
    /// Number of Input/Output (I/O) operations per second to provision for an `io1` or `io2` volume.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Option<i32>,
    /// Amazon Resource Name (ARN) of the Key Management Service (KMS) Key for encryption.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Identifier of the EC2 Volume Snapshot.
    #[builder(into)]
    #[serde(rename = "snapshotId")]
    pub r#snapshot_id: Option<String>,
    /// For GP3 volumes only. The throughput in MiB/s that the volume supports.
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: Option<i32>,
    /// Size of the volume, in GiB.
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Option<i32>,
    /// Type of the volume. For example, `gp2` or `io2`.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Option<String>,
}
