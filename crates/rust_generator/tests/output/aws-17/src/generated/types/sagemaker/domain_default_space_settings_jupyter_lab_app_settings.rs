#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainDefaultSpaceSettingsJupyterLabAppSettings {
    /// Indicates whether idle shutdown is activated for JupyterLab applications. see `app_lifecycle_management` Block below.
    #[builder(into)]
    #[serde(rename = "appLifecycleManagement")]
    pub r#app_lifecycle_management: Box<Option<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsAppLifecycleManagement>>,
    /// The lifecycle configuration that runs before the default lifecycle configuration. It can override changes made in the default lifecycle configuration.
    #[builder(into)]
    #[serde(rename = "builtInLifecycleConfigArn")]
    pub r#built_in_lifecycle_config_arn: Option<String>,
    /// A list of Git repositories that SageMaker automatically displays to users for cloning in the JupyterServer application. see `code_repository` Block below.
    #[builder(into)]
    #[serde(rename = "codeRepositories")]
    pub r#code_repositories: Option<Vec<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsCodeRepository>>,
    /// A list of custom SageMaker images that are configured to run as a JupyterLab app. see `custom_image` Block below.
    #[builder(into)]
    #[serde(rename = "customImages")]
    pub r#custom_images: Option<Vec<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsCustomImage>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. see `default_resource_spec` Block below.
    #[builder(into)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<Option<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsDefaultResourceSpec>>,
    /// The configuration parameters that specify the IAM roles assumed by the execution role of SageMaker (assumable roles) and the cluster instances or job execution environments (execution roles or runtime roles) to manage and access resources required for running Amazon EMR clusters or Amazon EMR Serverless applications. see `emr_settings` Block below.
    #[builder(into)]
    #[serde(rename = "emrSettings")]
    pub r#emr_settings: Box<Option<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettingsEmrSettings>>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configurations.
    #[builder(into)]
    #[serde(rename = "lifecycleConfigArns")]
    pub r#lifecycle_config_arns: Option<Vec<String>>,
}
