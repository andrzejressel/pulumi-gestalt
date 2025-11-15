#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomTargetTypeCustomActionsIncludeSkaffoldModuleGit {
    /// Relative path from the repository root to the Skaffold file.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Git ref the package should be cloned from.
    #[builder(into)]
    #[serde(rename = "ref")]
    pub r#ref_: Option<String>,
    /// Git repository the package should be cloned from.
    #[builder(into)]
    #[serde(rename = "repo")]
    pub r#repo: String,
}
