#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthConfigDecryptedCredentialJwt {
    /// (Output)
    /// The token calculated by the header, payload and signature.
    #[builder(into)]
    #[serde(rename = "jwt")]
    pub r#jwt: Option<String>,
    /// Identifies which algorithm is used to generate the signature.
    #[builder(into)]
    #[serde(rename = "jwtHeader")]
    pub r#jwt_header: Option<String>,
    /// Contains a set of claims. The JWT specification defines seven Registered Claim Names which are the standard fields commonly included in tokens. Custom claims are usually also included, depending on the purpose of the token.
    #[builder(into)]
    #[serde(rename = "jwtPayload")]
    pub r#jwt_payload: Option<String>,
    /// User's pre-shared secret to sign the token.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Option<String>,
}
