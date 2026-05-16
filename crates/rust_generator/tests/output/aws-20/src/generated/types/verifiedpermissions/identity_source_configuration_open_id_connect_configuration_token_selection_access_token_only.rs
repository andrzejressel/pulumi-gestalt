#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnly {
    /// The access token aud claim values that you want to accept in your policy store.
    #[builder(into)]
    #[serde(rename = "audiences")]
    pub r#audiences: Option<Vec<String>>,
    /// The claim that determines the principal in OIDC access tokens.
    #[builder(into)]
    #[serde(rename = "principalIdClaim")]
    pub r#principal_id_claim: Option<String>,
}
