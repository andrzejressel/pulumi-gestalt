#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApiOpenidAuthentication {
    /// How to send token to the server. A list of zero or more methods. Valid values are `authorizationHeader` and `query`.
    #[builder(into)]
    #[serde(rename = "bearerTokenSendingMethods")]
    pub r#bearer_token_sending_methods: Option<Vec<String>>,
    /// OpenID Connect provider identifier. The name of an OpenID Connect Provider.
    #[builder(into)]
    #[serde(rename = "openidProviderName")]
    pub r#openid_provider_name: String,
}
