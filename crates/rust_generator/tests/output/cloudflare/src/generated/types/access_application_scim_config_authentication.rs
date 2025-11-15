#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessApplicationScimConfigAuthentication {
    /// URL used to generate the auth code used during token generation. Required when using `scim_config.0.authentication.0.client_secret`, `scim_config.0.authentication.0.client_id`, `scim_config.0.authentication.0.token_url`. Conflicts with `scim_config.0.authentication.0.user`, `scim_config.0.authentication.0.password`, `scim_config.0.authentication.0.token`.
    #[builder(into)]
    #[serde(rename = "authorizationUrl")]
    pub r#authorization_url: Option<String>,
    /// Client ID used to authenticate when generating a token for authenticating with the remote SCIM service. Required when using `scim_config.0.authentication.0.client_secret`, `scim_config.0.authentication.0.authorization_url`, `scim_config.0.authentication.0.token_url`. Conflicts with `scim_config.0.authentication.0.user`, `scim_config.0.authentication.0.password`, `scim_config.0.authentication.0.token`.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// Secret used to authenticate when generating a token for authenticating with the remove SCIM service. Required when using `scim_config.0.authentication.0.client_id`, `scim_config.0.authentication.0.authorization_url`, `scim_config.0.authentication.0.token_url`. Conflicts with `scim_config.0.authentication.0.user`, `scim_config.0.authentication.0.password`, `scim_config.0.authentication.0.token`.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    /// Required when using `scim_config.0.authentication.0.user`. Conflicts with `scim_config.0.authentication.0.token`, `scim_config.0.authentication.0.client_id`, `scim_config.0.authentication.0.client_secret`, `scim_config.0.authentication.0.authorization_url`, `scim_config.0.authentication.0.token_url`, `scim_config.0.authentication.0.scopes`.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// The authentication scheme to use when making SCIM requests to this application.
    #[builder(into)]
    #[serde(rename = "scheme")]
    pub r#scheme: String,
    /// The authorization scopes to request when generating the token used to authenticate with the remove SCIM service. Conflicts with `scim_config.0.authentication.0.user`, `scim_config.0.authentication.0.password`, `scim_config.0.authentication.0.token`.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
    /// Token used to authenticate with the remote SCIM service. Conflicts with `scim_config.0.authentication.0.user`, `scim_config.0.authentication.0.password`, `scim_config.0.authentication.0.client_id`, `scim_config.0.authentication.0.client_secret`, `scim_config.0.authentication.0.authorization_url`, `scim_config.0.authentication.0.token_url`, `scim_config.0.authentication.0.scopes`.
    #[builder(into)]
    #[serde(rename = "token")]
    pub r#token: Option<String>,
    /// URL used to generate the token used to authenticate with the remote SCIM service. Required when using `scim_config.0.authentication.0.client_secret`, `scim_config.0.authentication.0.authorization_url`, `scim_config.0.authentication.0.client_id`. Conflicts with `scim_config.0.authentication.0.user`, `scim_config.0.authentication.0.password`, `scim_config.0.authentication.0.token`.
    #[builder(into)]
    #[serde(rename = "tokenUrl")]
    pub r#token_url: Option<String>,
    /// User name used to authenticate with the remote SCIM service. Required when using `scim_config.0.authentication.0.password`. Conflicts with `scim_config.0.authentication.0.token`, `scim_config.0.authentication.0.client_id`, `scim_config.0.authentication.0.client_secret`, `scim_config.0.authentication.0.authorization_url`, `scim_config.0.authentication.0.token_url`, `scim_config.0.authentication.0.scopes`.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Option<String>,
}
