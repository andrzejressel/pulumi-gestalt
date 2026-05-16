#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessApplicationSaasAppHybridAndImplicitOptions {
    /// If true, the authorization endpoint will return an access token.
    #[builder(into)]
    #[serde(rename = "returnAccessTokenFromAuthorizationEndpoint")]
    pub r#return_access_token_from_authorization_endpoint: Option<bool>,
    /// If true, the authorization endpoint will return an id token.
    #[builder(into)]
    #[serde(rename = "returnIdTokenFromAuthorizationEndpoint")]
    pub r#return_id_token_from_authorization_endpoint: Option<bool>,
}
