#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionAppSourceControl {
    /// The branch of the remote repository to use. Defaults to 'master'.
    #[builder(into, default)]
    #[serde(rename = "branch")]
    pub r#branch: Box<Option<String>>,
    /// Limits to manual integration. Defaults to `false` if not specified.
    #[builder(into, default)]
    #[serde(rename = "manualIntegration")]
    pub r#manual_integration: Box<Option<bool>>,
    /// The URL of the source code repository.
    #[builder(into, default)]
    #[serde(rename = "repoUrl")]
    pub r#repo_url: Box<Option<String>>,
    /// Enable roll-back for the repository. Defaults to `false` if not specified.
    #[builder(into, default)]
    #[serde(rename = "rollbackEnabled")]
    pub r#rollback_enabled: Box<Option<bool>>,
    /// Use Mercurial if `true`, otherwise uses Git.
    #[builder(into, default)]
    #[serde(rename = "useMercurial")]
    pub r#use_mercurial: Box<Option<bool>>,
}
