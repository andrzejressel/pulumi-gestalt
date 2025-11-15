#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpaceSpaceSettingsKernelGatewayAppSettings {
    /// A list of custom SageMaker images that are configured to run as a KernelGateway app. See `custom_image` Block below.
    #[builder(into)]
    #[serde(rename = "customImages")]
    pub r#custom_images: Option<Vec<super::super::types::sagemaker::SpaceSpaceSettingsKernelGatewayAppSettingsCustomImage>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. See `default_resource_spec` Block below.
    #[builder(into)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<super::super::types::sagemaker::SpaceSpaceSettingsKernelGatewayAppSettingsDefaultResourceSpec>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configurations.
    #[builder(into)]
    #[serde(rename = "lifecycleConfigArns")]
    pub r#lifecycle_config_arns: Option<Vec<String>>,
}
