#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLaunchTemplateBlockDeviceMapping {
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: String,
    #[builder(into)]
    #[serde(rename = "ebs")]
    pub r#ebs: Vec<super::super::types::ec2::GetLaunchTemplateBlockDeviceMappingEb>,
    #[builder(into)]
    #[serde(rename = "noDevice")]
    pub r#no_device: String,
    #[builder(into)]
    #[serde(rename = "virtualName")]
    pub r#virtual_name: String,
}
