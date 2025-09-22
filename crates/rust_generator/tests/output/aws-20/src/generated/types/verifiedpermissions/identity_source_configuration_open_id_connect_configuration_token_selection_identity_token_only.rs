#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionIdentityTokenOnly {
    /// The ID token audience, or client ID, claim values that you want to accept in your policy store from an OIDC identity provider.
    #[builder(into)]
    #[serde(rename = "clientIds")]
    pub r#client_ids: Option<Vec<String>>,
    /// The claim that determines the principal in OIDC access tokens.
    #[builder(into)]
    #[serde(rename = "principalIdClaim")]
    pub r#principal_id_claim: Option<String>,
}
