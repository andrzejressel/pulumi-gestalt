#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListenerDefaultActionAuthenticateOidc {
    /// Query parameters to include in the redirect request to the authorization endpoint. Max: 10.
    #[builder(into)]
    #[serde(rename = "authenticationRequestExtraParams")]
    pub r#authentication_request_extra_params: Option<std::collections::HashMap<String, String>>,
    /// Authorization endpoint of the IdP.
    #[builder(into)]
    #[serde(rename = "authorizationEndpoint")]
    pub r#authorization_endpoint: String,
    /// OAuth 2.0 client identifier.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// OAuth 2.0 client secret.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: String,
    /// OIDC issuer identifier of the IdP.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: String,
    /// Behavior if the user is not authenticated. Valid values: `deny`, `allow` and `authenticate`
    #[builder(into)]
    #[serde(rename = "onUnauthenticatedRequest")]
    pub r#on_unauthenticated_request: Option<String>,
    /// Set of user claims to be requested from the IdP.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Option<String>,
    /// Name of the cookie used to maintain session information.
    #[builder(into)]
    #[serde(rename = "sessionCookieName")]
    pub r#session_cookie_name: Option<String>,
    /// Maximum duration of the authentication session, in seconds.
    #[builder(into)]
    #[serde(rename = "sessionTimeout")]
    pub r#session_timeout: Option<i32>,
    /// Token endpoint of the IdP.
    #[builder(into)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: String,
    /// User info endpoint of the IdP.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "userInfoEndpoint")]
    pub r#user_info_endpoint: String,
}
