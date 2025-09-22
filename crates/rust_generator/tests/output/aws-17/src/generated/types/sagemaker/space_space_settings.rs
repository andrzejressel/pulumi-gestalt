#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpaceSpaceSettings {
    /// The type of app created within the space.
    #[builder(into)]
    #[serde(rename = "appType")]
    pub r#app_type: Option<String>,
    /// The Code Editor application settings. See `code_editor_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "codeEditorAppSettings")]
    pub r#code_editor_app_settings: Option<Box<super::super::types::sagemaker::SpaceSpaceSettingsCodeEditorAppSettings>>,
    /// A file system, created by you, that you assign to a space for an Amazon SageMaker Domain. See `custom_file_system` Block below.
    #[builder(into)]
    #[serde(rename = "customFileSystems")]
    pub r#custom_file_systems: Option<Vec<super::super::types::sagemaker::SpaceSpaceSettingsCustomFileSystem>>,
    /// The settings for the JupyterLab application. See `jupyter_lab_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "jupyterLabAppSettings")]
    pub r#jupyter_lab_app_settings: Option<Box<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettings>>,
    /// The Jupyter server's app settings. See `jupyter_server_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "jupyterServerAppSettings")]
    pub r#jupyter_server_app_settings: Option<Box<super::super::types::sagemaker::SpaceSpaceSettingsJupyterServerAppSettings>>,
    /// The kernel gateway app settings. See `kernel_gateway_app_settings` Block below.
    #[builder(into)]
    #[serde(rename = "kernelGatewayAppSettings")]
    pub r#kernel_gateway_app_settings: Option<Box<super::super::types::sagemaker::SpaceSpaceSettingsKernelGatewayAppSettings>>,
    /// The storage settings. See `space_storage_settings` Block below.
    #[builder(into)]
    #[serde(rename = "spaceStorageSettings")]
    pub r#space_storage_settings: Option<Box<super::super::types::sagemaker::SpaceSpaceSettingsSpaceStorageSettings>>,
}
