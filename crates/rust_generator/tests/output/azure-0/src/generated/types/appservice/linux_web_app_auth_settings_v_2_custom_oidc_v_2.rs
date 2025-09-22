#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinuxWebAppAuthSettingsV2CustomOidcV2 {
    /// The endpoint to make the Authorisation Request as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "authorisationEndpoint")]
    pub r#authorisation_endpoint: Option<String>,
    /// The endpoint that provides the keys necessary to validate the token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "certificationUri")]
    pub r#certification_uri: Option<String>,
    /// The Client Credential Method used.
    #[builder(into)]
    #[serde(rename = "clientCredentialMethod")]
    pub r#client_credential_method: Option<String>,
    /// The ID of the Client to use to authenticate with the Custom OIDC.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The App Setting name that contains the secret for this Custom OIDC Client. This is generated from `name` above and suffixed with `_PROVIDER_AUTHENTICATION_SECRET`.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Option<String>,
    /// The endpoint that issued the Token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "issuerEndpoint")]
    pub r#issuer_endpoint: Option<String>,
    /// The name of the Custom OIDC Authentication Provider.
    /// 
    /// > **NOTE:** An `app_setting` matching this value in upper case with the suffix of `_PROVIDER_AUTHENTICATION_SECRET` is required. e.g. `MYOIDC_PROVIDER_AUTHENTICATION_SECRET` for a value of `myoidc`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The name of the claim that contains the users name.
    #[builder(into)]
    #[serde(rename = "nameClaimType")]
    pub r#name_claim_type: Option<String>,
    /// Specifies the endpoint used for OpenID Connect Discovery. For example `https://example.com/.well-known/openid-configuration`.
    #[builder(into)]
    #[serde(rename = "openidConfigurationEndpoint")]
    pub r#openid_configuration_endpoint: String,
    /// The list of the scopes that should be requested while authenticating.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
    /// The endpoint used to request a Token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: Option<String>,
}
