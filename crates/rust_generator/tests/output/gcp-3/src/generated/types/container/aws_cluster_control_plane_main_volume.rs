#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AwsClusterControlPlaneMainVolume {
    /// Optional. The number of I/O operations per second (IOPS) to provision for GP3 volume.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Option<i32>,
    /// Optional. The Amazon Resource Name (ARN) of the Customer Managed Key (CMK) used to encrypt AWS EBS volumes. If not specified, the default Amazon managed key associated to the AWS region where this cluster runs will be used.
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Option<String>,
    /// Optional. The size of the volume, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource.
    #[builder(into)]
    #[serde(rename = "sizeGib")]
    pub r#size_gib: Option<i32>,
    /// Optional. The throughput to provision for the volume, in MiB/s. Only valid if the volume type is GP3. If volume type is gp3 and throughput is not specified, the throughput will defaults to 125.
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: Option<i32>,
    /// Optional. Type of the EBS volume. When unspecified, it defaults to GP2 volume. Possible values: VOLUME_TYPE_UNSPECIFIED, GP2, GP3
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Option<String>,
}
