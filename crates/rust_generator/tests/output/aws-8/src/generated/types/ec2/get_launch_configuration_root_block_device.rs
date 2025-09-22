#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLaunchConfigurationRootBlockDevice {
    /// Whether the EBS Volume will be deleted on instance termination.
    #[builder(into)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: bool,
    /// Whether the volume is Encrypted.
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: bool,
    /// Provisioned IOPs of the volume.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: i32,
    /// Throughput of the volume.
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: i32,
    /// Size of the volume.
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: i32,
    /// Type of the volume.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: String,
}
