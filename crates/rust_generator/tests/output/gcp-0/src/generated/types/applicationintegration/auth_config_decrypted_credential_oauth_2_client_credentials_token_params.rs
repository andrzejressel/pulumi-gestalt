#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthConfigDecryptedCredentialOauth2ClientCredentialsTokenParams {
    /// A list of parameter map entries.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "entries")]
    pub r#entries: Option<Vec<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2ClientCredentialsTokenParamsEntry>>,
}
