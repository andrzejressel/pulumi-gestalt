#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContainerRecipeInstanceConfigurationBlockDeviceMapping {
    /// Name of the device. For example, `/dev/sda` or `/dev/xvdb`.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Option<String>,
    /// Configuration block with Elastic Block Storage (EBS) block device mapping settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "ebs")]
    pub r#ebs: Option<Box<super::super::types::imagebuilder::ContainerRecipeInstanceConfigurationBlockDeviceMappingEbs>>,
    /// Set to `true` to remove a mapping from the parent image.
    #[builder(into)]
    #[serde(rename = "noDevice")]
    pub r#no_device: Option<bool>,
    /// Virtual device name. For example, `ephemeral0`. Instance store volumes are numbered starting from 0.
    #[builder(into)]
    #[serde(rename = "virtualName")]
    pub r#virtual_name: Option<String>,
}
