#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionAuthConfigOauth2JwtBearerJwtClaims {
    /// Value for the "aud" claim.
    /// 
    /// <a name="nested_oauth2_client_credentials"></a>The `oauth2_client_credentials` block supports:
    #[builder(into)]
    #[serde(rename = "audience")]
    pub r#audience: Option<String>,
    /// Value for the "iss" claim.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Option<String>,
    /// Value for the "sub" claim.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
}
