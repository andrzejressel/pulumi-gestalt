#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskDefinitionVolumeFsxWindowsFileServerVolumeConfiguration {
    /// Configuration block for authorization for the Amazon FSx for Windows File Server file system detailed below.
    #[builder(into)]
    #[serde(rename = "authorizationConfig")]
    pub r#authorization_config: Box<super::super::types::ecs::TaskDefinitionVolumeFsxWindowsFileServerVolumeConfigurationAuthorizationConfig>,
    /// The Amazon FSx for Windows File Server file system ID to use.
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    pub r#file_system_id: Box<String>,
    /// The directory within the Amazon FSx for Windows File Server file system to mount as the root directory inside the host.
    #[builder(into)]
    #[serde(rename = "rootDirectory")]
    pub r#root_directory: Box<String>,
}
