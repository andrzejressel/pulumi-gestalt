#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryGitRemoteSettings {
    /// The name of the Secret Manager secret version to use as an authentication token for Git operations. This secret is for assigning with HTTPS only(for SSH use `ssh_authentication_config`). Must be in the format projects/*/secrets/*/versions/*.
    #[builder(into)]
    #[serde(rename = "authenticationTokenSecretVersion")]
    pub r#authentication_token_secret_version: Option<String>,
    /// The Git remote's default branch name.
    #[builder(into)]
    #[serde(rename = "defaultBranch")]
    pub r#default_branch: String,
    /// Authentication fields for remote uris using SSH protocol.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sshAuthenticationConfig")]
    pub r#ssh_authentication_config: Option<Box<super::super::types::dataform::RepositoryGitRemoteSettingsSshAuthenticationConfig>>,
    /// (Output)
    /// Indicates the status of the Git access token. https://cloud.google.com/dataform/reference/rest/v1beta1/projects.locations.repositories#TokenStatus
    #[builder(into)]
    #[serde(rename = "tokenStatus")]
    pub r#token_status: Option<String>,
    /// The Git remote's URL.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}
