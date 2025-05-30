#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomTargetTypeCustomActionsIncludeSkaffoldModuleGoogleCloudBuildRepo {
    /// Relative path from the repository root to the Skaffold file.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Branch or tag to use when cloning the repository.
    #[builder(into, default)]
    #[serde(rename = "ref")]
    pub r#ref_: Box<Option<String>>,
    /// Cloud Build 2nd gen repository in the format of 'projects/<project>/locations/<location>/connections/<connection>/repositories/<repository>'.
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: Box<String>,
}
