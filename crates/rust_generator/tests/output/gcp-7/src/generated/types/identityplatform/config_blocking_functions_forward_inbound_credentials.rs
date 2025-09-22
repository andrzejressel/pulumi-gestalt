#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigBlockingFunctionsForwardInboundCredentials {
    /// Whether to pass the user's OAuth identity provider's access token.
    #[builder(into)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Option<bool>,
    /// Whether to pass the user's OIDC identity provider's ID token.
    #[builder(into)]
    #[serde(rename = "idToken")]
    pub r#id_token: Option<bool>,
    /// Whether to pass the user's OAuth identity provider's refresh token.
    #[builder(into)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Option<bool>,
}
