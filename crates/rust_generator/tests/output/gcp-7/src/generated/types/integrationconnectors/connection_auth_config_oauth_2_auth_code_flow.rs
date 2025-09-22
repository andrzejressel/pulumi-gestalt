#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionAuthConfigOauth2AuthCodeFlow {
    /// Auth URL for Authorization Code Flow.
    #[builder(into)]
    #[serde(rename = "authUri")]
    pub r#auth_uri: Option<String>,
    /// Client ID for user-provided OAuth app.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// Client secret for user-provided OAuth app.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2AuthCodeFlowClientSecret>>,
    /// Whether to enable PKCE when the user performs the auth code flow.
    #[builder(into)]
    #[serde(rename = "enablePkce")]
    pub r#enable_pkce: Option<bool>,
    /// Scopes the connection will request when the user performs the auth code flow.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
}
