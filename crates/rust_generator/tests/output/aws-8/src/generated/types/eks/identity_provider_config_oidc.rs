#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IdentityProviderConfigOidc {
    /// Client ID for the OpenID Connect identity provider.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The JWT claim that the provider will use to return groups.
    #[builder(into)]
    #[serde(rename = "groupsClaim")]
    pub r#groups_claim: Option<String>,
    /// A prefix that is prepended to group claims e.g., `oidc:`.
    #[builder(into)]
    #[serde(rename = "groupsPrefix")]
    pub r#groups_prefix: Option<String>,
    /// The name of the identity provider config.
    #[builder(into)]
    #[serde(rename = "identityProviderConfigName")]
    pub r#identity_provider_config_name: String,
    /// Issuer URL for the OpenID Connect identity provider.
    #[builder(into)]
    #[serde(rename = "issuerUrl")]
    pub r#issuer_url: String,
    /// The key value pairs that describe required claims in the identity token.
    #[builder(into)]
    #[serde(rename = "requiredClaims")]
    pub r#required_claims: Option<std::collections::HashMap<String, String>>,
    /// The JWT claim that the provider will use as the username.
    #[builder(into)]
    #[serde(rename = "usernameClaim")]
    pub r#username_claim: Option<String>,
    /// A prefix that is prepended to username claims.
    #[builder(into)]
    #[serde(rename = "usernamePrefix")]
    pub r#username_prefix: Option<String>,
}
