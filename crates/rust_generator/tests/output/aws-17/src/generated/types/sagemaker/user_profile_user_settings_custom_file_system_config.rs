#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserProfileUserSettingsCustomFileSystemConfig {
    /// The default EBS storage settings for a private space. See EFS File System Config below.
    #[builder(into)]
    #[serde(rename = "efsFileSystemConfigs")]
    pub r#efs_file_system_configs: Option<Vec<super::super::types::sagemaker::UserProfileUserSettingsCustomFileSystemConfigEfsFileSystemConfig>>,
}
