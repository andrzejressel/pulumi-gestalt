#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessIdentityProviderConfig {
    #[builder(into)]
    #[serde(rename = "apiToken")]
    pub r#api_token: Option<String>,
    #[builder(into)]
    #[serde(rename = "appsDomain")]
    pub r#apps_domain: Option<String>,
    #[builder(into)]
    #[serde(rename = "attributes")]
    pub r#attributes: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "authUrl")]
    pub r#auth_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "authorizationServerId")]
    pub r#authorization_server_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "centrifyAccount")]
    pub r#centrify_account: Option<String>,
    #[builder(into)]
    #[serde(rename = "centrifyAppId")]
    pub r#centrify_app_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "certsUrl")]
    pub r#certs_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "claims")]
    pub r#claims: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    #[builder(into)]
    #[serde(rename = "conditionalAccessEnabled")]
    pub r#conditional_access_enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "directoryId")]
    pub r#directory_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "emailAttributeName")]
    pub r#email_attribute_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "emailClaimName")]
    pub r#email_claim_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "idpPublicCert")]
    pub r#idp_public_cert: Option<String>,
    #[builder(into)]
    #[serde(rename = "issuerUrl")]
    pub r#issuer_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "oktaAccount")]
    pub r#okta_account: Option<String>,
    #[builder(into)]
    #[serde(rename = "oneloginAccount")]
    pub r#onelogin_account: Option<String>,
    #[builder(into)]
    #[serde(rename = "pingEnvId")]
    pub r#ping_env_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "pkceEnabled")]
    pub r#pkce_enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "redirectUrl")]
    pub r#redirect_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "signRequest")]
    pub r#sign_request: Option<bool>,
    #[builder(into)]
    #[serde(rename = "ssoTargetUrl")]
    pub r#sso_target_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "supportGroups")]
    pub r#support_groups: Option<bool>,
    #[builder(into)]
    #[serde(rename = "tokenUrl")]
    pub r#token_url: Option<String>,
}
