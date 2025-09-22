#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthConfigDecryptedCredential {
    /// Auth token credential.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "authToken")]
    pub r#auth_token: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialAuthToken>>,
    /// Credential type associated with auth configs.
    #[builder(into)]
    #[serde(rename = "credentialType")]
    pub r#credential_type: String,
    /// JWT credential.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "jwt")]
    pub r#jwt: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialJwt>>,
    /// OAuth2 authorization code credential.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oauth2AuthorizationCode")]
    pub r#oauth_2_authorization_code: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2AuthorizationCode>>,
    /// OAuth2 client credentials.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oauth2ClientCredentials")]
    pub r#oauth_2_client_credentials: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2ClientCredentials>>,
    /// Google OIDC ID Token.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oidcToken")]
    pub r#oidc_token: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOidcToken>>,
    /// Service account credential.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "serviceAccountCredentials")]
    pub r#service_account_credentials: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialServiceAccountCredentials>>,
    /// Username and password credential.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "usernameAndPassword")]
    pub r#username_and_password: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialUsernameAndPassword>>,
}
