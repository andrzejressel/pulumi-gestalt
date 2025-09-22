#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserProfileUserSettingsCodeEditorAppSettings {
    /// Indicates whether idle shutdown is activated for JupyterLab applications. see `app_lifecycle_management` Block below.
    #[builder(into)]
    #[serde(rename = "appLifecycleManagement")]
    pub r#app_lifecycle_management: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsCodeEditorAppSettingsAppLifecycleManagement>>,
    /// The lifecycle configuration that runs before the default lifecycle configuration. It can override changes made in the default lifecycle configuration.
    #[builder(into)]
    #[serde(rename = "builtInLifecycleConfigArn")]
    pub r#built_in_lifecycle_config_arn: Option<String>,
    /// A list of custom SageMaker images that are configured to run as a CodeEditor app. see Custom Image below.
    #[builder(into)]
    #[serde(rename = "customImages")]
    pub r#custom_images: Option<Vec<super::super::types::sagemaker::UserProfileUserSettingsCodeEditorAppSettingsCustomImage>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. see Default Resource Spec below.
    #[builder(into)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsCodeEditorAppSettingsDefaultResourceSpec>>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configurations.
    #[builder(into)]
    #[serde(rename = "lifecycleConfigArns")]
    pub r#lifecycle_config_arns: Option<Vec<String>>,
}
