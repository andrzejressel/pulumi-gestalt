#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryRemoteRepositoryConfigUpstreamCredentials {
    /// Use username and password to access the remote repository.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "usernamePasswordCredentials")]
    pub r#username_password_credentials: Option<Box<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigUpstreamCredentialsUsernamePasswordCredentials>>,
}
