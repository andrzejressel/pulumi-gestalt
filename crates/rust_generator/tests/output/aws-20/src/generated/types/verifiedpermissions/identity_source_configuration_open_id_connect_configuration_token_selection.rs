#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelection {
    /// The OIDC configuration for processing access tokens. See Access Token Only below.
    #[builder(into)]
    #[serde(rename = "accessTokenOnly")]
    pub r#access_token_only: Box<Option<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnly>>,
    /// The OIDC configuration for processing identity (ID) tokens. See Identity Token Only below.
    #[builder(into)]
    #[serde(rename = "identityTokenOnly")]
    pub r#identity_token_only: Box<Option<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionIdentityTokenOnly>>,
}
