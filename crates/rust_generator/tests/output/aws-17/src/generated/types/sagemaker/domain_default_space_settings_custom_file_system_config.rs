#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDefaultSpaceSettingsCustomFileSystemConfig {
    /// The default EBS storage settings for a private space. See `efs_file_system_config` Block below.
    #[builder(into)]
    #[serde(rename = "efsFileSystemConfig")]
    pub r#efs_file_system_config: Option<Box<super::super::types::sagemaker::DomainDefaultSpaceSettingsCustomFileSystemConfigEfsFileSystemConfig>>,
}
