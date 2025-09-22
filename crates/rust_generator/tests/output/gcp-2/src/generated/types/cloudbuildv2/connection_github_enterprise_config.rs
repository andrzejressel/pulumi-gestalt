#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionGithubEnterpriseConfig {
    /// Id of the GitHub App created from the manifest.
    #[builder(into)]
    #[serde(rename = "appId")]
    pub r#app_id: Option<i32>,
    /// ID of the installation of the GitHub App.
    #[builder(into)]
    #[serde(rename = "appInstallationId")]
    pub r#app_installation_id: Option<i32>,
    /// The URL-friendly name of the GitHub App.
    #[builder(into)]
    #[serde(rename = "appSlug")]
    pub r#app_slug: Option<String>,
    /// Required. The URI of the GitHub Enterprise host this connection is for.
    #[builder(into)]
    #[serde(rename = "hostUri")]
    pub r#host_uri: String,
    /// SecretManager resource containing the private key of the GitHub App, formatted as `projects/*/secrets/*/versions/*`.
    #[builder(into)]
    #[serde(rename = "privateKeySecretVersion")]
    pub r#private_key_secret_version: Option<String>,
    /// Configuration for using Service Directory to privately connect to a GitHub Enterprise server. This should only be set if the GitHub Enterprise server is hosted on-premises and not reachable by public internet. If this field is left empty, calls to the GitHub Enterprise server will be made over the public internet.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "serviceDirectoryConfig")]
    pub r#service_directory_config: Box<Option<super::super::types::cloudbuildv2::ConnectionGithubEnterpriseConfigServiceDirectoryConfig>>,
    /// SSL certificate to use for requests to GitHub Enterprise.
    #[builder(into)]
    #[serde(rename = "sslCa")]
    pub r#ssl_ca: Option<String>,
    /// SecretManager resource containing the webhook secret of the GitHub App, formatted as `projects/*/secrets/*/versions/*`.
    #[builder(into)]
    #[serde(rename = "webhookSecretSecretVersion")]
    pub r#webhook_secret_secret_version: Option<String>,
}
