#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSpringCloudServiceConfigServerGitSettingRepository {
    /// A `http_basic_auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "httpBasicAuths")]
    pub r#http_basic_auths: Vec<super::super::types::appplatform::GetSpringCloudServiceConfigServerGitSettingRepositoryHttpBasicAuth>,
    /// The default label of the Git repository, which is a branch name, tag name, or commit-id of the repository
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: String,
    /// Specifies The name of the Spring Cloud Service resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// An array of strings used to match an application name. For each pattern, use the `{application}/{profile}` format with wildcards.
    #[builder(into)]
    #[serde(rename = "patterns")]
    pub r#patterns: Vec<String>,
    /// An array of strings used to search subdirectories of the Git repository.
    #[builder(into)]
    #[serde(rename = "searchPaths")]
    pub r#search_paths: Vec<String>,
    /// A `ssh_auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "sshAuths")]
    pub r#ssh_auths: Vec<super::super::types::appplatform::GetSpringCloudServiceConfigServerGitSettingRepositorySshAuth>,
    /// The URI of the Git repository
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}
