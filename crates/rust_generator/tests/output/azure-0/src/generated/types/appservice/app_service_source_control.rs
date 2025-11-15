#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppServiceSourceControl {
    /// The branch of the remote repository to use. Defaults to 'master'.
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Option<String>,
    /// Limits to manual integration. Defaults to `false` if not specified.
    #[builder(into)]
    #[serde(rename = "manualIntegration")]
    pub r#manual_integration: Option<bool>,
    /// The URL of the source code repository.
    #[builder(into)]
    #[serde(rename = "repoUrl")]
    pub r#repo_url: Option<String>,
    /// Enable roll-back for the repository. Defaults to `false` if not specified.
    #[builder(into)]
    #[serde(rename = "rollbackEnabled")]
    pub r#rollback_enabled: Option<bool>,
    /// Use Mercurial if `true`, otherwise uses Git.
    #[builder(into)]
    #[serde(rename = "useMercurial")]
    pub r#use_mercurial: Option<bool>,
}
