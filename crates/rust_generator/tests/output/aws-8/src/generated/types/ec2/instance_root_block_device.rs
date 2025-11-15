#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceRootBlockDevice {
    /// Whether the volume should be destroyed on instance termination. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Option<bool>,
    /// Device name, e.g., `/dev/sdh` or `xvdh`.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Option<String>,
    /// Whether to enable volume encryption. Defaults to `false`. Must be configured to perform drift detection.
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Option<bool>,
    /// Amount of provisioned [IOPS](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-io-characteristics.html). Only valid for volume_type of `io1`, `io2` or `gp3`.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Option<i32>,
    /// Amazon Resource Name (ARN) of the KMS Key to use when encrypting the volume. Must be configured to perform drift detection.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Map of tags to assign to the device.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
    #[builder(into)]
    #[serde(rename = "tagsAll")]
    pub r#tags_all: Option<std::collections::HashMap<String, String>>,
    /// Throughput to provision for a volume in mebibytes per second (MiB/s). This is only valid for `volume_type` of `gp3`.
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: Option<i32>,
    /// ID of the volume. For example, the ID can be accessed like this, `aws_instance.web.root_block_device.0.volume_id`.
    #[builder(into)]
    #[serde(rename = "volumeId")]
    pub r#volume_id: Option<String>,
    /// Size of the volume in gibibytes (GiB).
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Option<i32>,
    /// Type of volume. Valid values include `standard`, `gp2`, `gp3`, `io1`, `io2`, `sc1`, or `st1`. Defaults to the volume type that the AMI uses.
    /// 
    /// Modifying the `encrypted` or `kms_key_id` settings of the `root_block_device` requires resource replacement.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Option<String>,
}
