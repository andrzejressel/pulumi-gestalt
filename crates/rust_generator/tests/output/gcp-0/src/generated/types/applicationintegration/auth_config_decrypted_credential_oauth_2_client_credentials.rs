#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthConfigDecryptedCredentialOauth2ClientCredentials {
    /// The client's ID.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// The client's secret.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    /// Represent how to pass parameters to fetch access token Possible values: ["REQUEST_TYPE_UNSPECIFIED", "REQUEST_BODY", "QUERY_PARAMETERS", "ENCODED_HEADER"]
    #[builder(into)]
    #[serde(rename = "requestType")]
    pub r#request_type: Option<String>,
    /// A space-delimited list of requested scope permissions.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Option<String>,
    /// The token endpoint is used by the client to obtain an access token by presenting its authorization grant or refresh token.
    #[builder(into)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: Option<String>,
    /// Token parameters for the auth request.
    #[builder(into)]
    #[serde(rename = "tokenParams")]
    pub r#token_params: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2ClientCredentialsTokenParams>>,
}
