#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomTargetTypeCustomActionsIncludeSkaffoldModule {
    /// The Skaffold Config modules to use from the specified source.
    #[builder(into)]
    #[serde(rename = "configs")]
    pub r#configs: Option<Vec<String>>,
    /// Remote git repository containing the Skaffold Config modules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "git")]
    pub r#git: Option<Box<super::super::types::clouddeploy::CustomTargetTypeCustomActionsIncludeSkaffoldModuleGit>>,
    /// Cloud Build 2nd gen repository containing the Skaffold Config modules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "googleCloudBuildRepo")]
    pub r#google_cloud_build_repo: Option<Box<super::super::types::clouddeploy::CustomTargetTypeCustomActionsIncludeSkaffoldModuleGoogleCloudBuildRepo>>,
    /// Cloud Storage bucket containing Skaffold Config modules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "googleCloudStorage")]
    pub r#google_cloud_storage: Option<Box<super::super::types::clouddeploy::CustomTargetTypeCustomActionsIncludeSkaffoldModuleGoogleCloudStorage>>,
}
