#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IdentityPoolCognitoIdentityProvider {
    /// The client ID for the Amazon Cognito Identity User Pool.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// The provider name for an Amazon Cognito Identity User Pool.
    #[builder(into)]
    #[serde(rename = "providerName")]
    pub r#provider_name: Option<String>,
    /// Whether server-side token validation is enabled for the identity provider’s token or not.
    #[builder(into)]
    #[serde(rename = "serverSideTokenCheck")]
    pub r#server_side_token_check: Option<bool>,
}
