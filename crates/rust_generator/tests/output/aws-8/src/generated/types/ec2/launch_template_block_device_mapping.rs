#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LaunchTemplateBlockDeviceMapping {
    /// The name of the device to mount.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Option<String>,
    /// Configure EBS volume properties.
    #[builder(into)]
    #[serde(rename = "ebs")]
    pub r#ebs: Option<Box<super::super::types::ec2::LaunchTemplateBlockDeviceMappingEbs>>,
    /// Suppresses the specified device included in the AMI's block device mapping.
    #[builder(into)]
    #[serde(rename = "noDevice")]
    pub r#no_device: Option<String>,
    /// The [Instance Store Device
    /// Name](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#InstanceStoreDeviceNames)
    /// (e.g., `"ephemeral0"`).
    #[builder(into)]
    #[serde(rename = "virtualName")]
    pub r#virtual_name: Option<String>,
}
