#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionGitlabConfig {
    /// Required. A GitLab personal access token with the `api` scope access.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "authorizerCredential")]
    pub r#authorizer_credential: Box<super::super::types::cloudbuildv2::ConnectionGitlabConfigAuthorizerCredential>,
    /// The URI of the GitLab Enterprise host this connection is for. If not specified, the default value is https://gitlab.com.
    #[builder(into)]
    #[serde(rename = "hostUri")]
    pub r#host_uri: Option<String>,
    /// Required. A GitLab personal access token with the minimum `read_api` scope access.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "readAuthorizerCredential")]
    pub r#read_authorizer_credential: Box<super::super::types::cloudbuildv2::ConnectionGitlabConfigReadAuthorizerCredential>,
    /// (Output)
    /// Output only. Version of the GitLab Enterprise server running on the `host_uri`.
    #[builder(into)]
    #[serde(rename = "serverVersion")]
    pub r#server_version: Option<String>,
    /// Configuration for using Service Directory to privately connect to a GitLab Enterprise server. This should only be set if the GitLab Enterprise server is hosted on-premises and not reachable by public internet. If this field is left empty, calls to the GitLab Enterprise server will be made over the public internet.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "serviceDirectoryConfig")]
    pub r#service_directory_config: Option<Box<super::super::types::cloudbuildv2::ConnectionGitlabConfigServiceDirectoryConfig>>,
    /// SSL certificate to use for requests to GitLab Enterprise.
    #[builder(into)]
    #[serde(rename = "sslCa")]
    pub r#ssl_ca: Option<String>,
    /// Required. Immutable. SecretManager resource containing the webhook secret of a GitLab Enterprise project, formatted as `projects/*/secrets/*/versions/*`.
    #[builder(into)]
    #[serde(rename = "webhookSecretSecretVersion")]
    pub r#webhook_secret_secret_version: String,
}
