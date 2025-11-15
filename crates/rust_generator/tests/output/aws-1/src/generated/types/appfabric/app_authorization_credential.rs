#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppAuthorizationCredential {
    /// Contains API key credential information.
    #[builder(into)]
    #[serde(rename = "apiKeyCredentials")]
    pub r#api_key_credentials: Option<Vec<super::super::types::appfabric::AppAuthorizationCredentialApiKeyCredential>>,
    /// Contains OAuth2 client credential information.
    #[builder(into)]
    #[serde(rename = "oauth2Credential")]
    pub r#oauth_2_credential: Option<Box<super::super::types::appfabric::AppAuthorizationCredentialOauth2Credential>>,
}
