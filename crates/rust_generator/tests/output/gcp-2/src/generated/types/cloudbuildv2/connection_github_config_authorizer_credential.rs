#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionGithubConfigAuthorizerCredential {
    /// A SecretManager resource containing the OAuth token that authorizes the Cloud Build connection. Format: `projects/*/secrets/*/versions/*`.
    #[builder(into)]
    #[serde(rename = "oauthTokenSecretVersion")]
    pub r#oauth_token_secret_version: Option<String>,
    /// (Output)
    /// Output only. The username associated to this token.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
