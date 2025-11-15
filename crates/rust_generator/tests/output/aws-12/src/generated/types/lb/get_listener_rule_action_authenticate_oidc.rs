#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerRuleActionAuthenticateOidc {
    /// Set of additional parameters for the request.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "authenticationRequestExtraParams")]
    pub r#authentication_request_extra_params: std::collections::HashMap<String, String>,
    /// The authorization endpoint of the IdP.
    #[builder(into)]
    #[serde(rename = "authorizationEndpoint")]
    pub r#authorization_endpoint: String,
    /// OAuth 2.0 client identifier.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// OIDC issuer identifier of the IdP.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: String,
    /// Behavior when the client is not authenticated.
    #[builder(into)]
    #[serde(rename = "onUnauthenticatedRequest")]
    pub r#on_unauthenticated_request: String,
    /// Set of user claims requested.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: String,
    /// Name of the cookie used to maintain session information.
    #[builder(into)]
    #[serde(rename = "sessionCookieName")]
    pub r#session_cookie_name: String,
    /// Maximum duration of the authentication session in seconds.
    #[builder(into)]
    #[serde(rename = "sessionTimeout")]
    pub r#session_timeout: i32,
    /// The token endpoint of the IdP.
    #[builder(into)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: String,
    /// The user info endpoint of the IdP.
    #[builder(into)]
    #[serde(rename = "userInfoEndpoint")]
    pub r#user_info_endpoint: String,
}
