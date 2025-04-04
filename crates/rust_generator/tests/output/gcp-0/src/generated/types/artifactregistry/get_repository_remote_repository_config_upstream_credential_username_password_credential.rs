#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRepositoryRemoteRepositoryConfigUpstreamCredentialUsernamePasswordCredential {
    /// The Secret Manager key version that holds the password to access the
    /// remote repository. Must be in the format of
    /// 'projects/{project}/secrets/{secret}/versions/{version}'.
    #[builder(into)]
    #[serde(rename = "passwordSecretVersion")]
    pub r#password_secret_version: Box<String>,
    /// The username to access the remote repository.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
