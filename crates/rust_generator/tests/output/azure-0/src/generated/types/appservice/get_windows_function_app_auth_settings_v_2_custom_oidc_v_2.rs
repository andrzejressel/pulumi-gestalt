#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWindowsFunctionAppAuthSettingsV2CustomOidcV2 {
    /// The endpoint to make the Authorisation Request as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "authorisationEndpoint")]
    pub r#authorisation_endpoint: String,
    /// The endpoint that provides the keys necessary to validate the token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "certificationUri")]
    pub r#certification_uri: String,
    /// The Client Credential Method used.
    #[builder(into)]
    #[serde(rename = "clientCredentialMethod")]
    pub r#client_credential_method: String,
    /// The OAuth 2.0 client ID that was created for the app used for authentication.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The app setting name containing the OAuth 2.0 client secret that was created for the app used for authentication.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: String,
    /// The endpoint that issued the Token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "issuerEndpoint")]
    pub r#issuer_endpoint: String,
    /// The name of this Windows Function App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The name of the claim that contains the users name.
    #[builder(into)]
    #[serde(rename = "nameClaimType")]
    pub r#name_claim_type: String,
    /// The endpoint used for OpenID Connect Discovery. For example `https://example.com/.well-known/openid-configuration`.
    #[builder(into)]
    #[serde(rename = "openidConfigurationEndpoint")]
    pub r#openid_configuration_endpoint: String,
    /// The list of the scopes that are requested while authenticating.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Vec<String>,
    /// The endpoint used to request a Token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: String,
}
