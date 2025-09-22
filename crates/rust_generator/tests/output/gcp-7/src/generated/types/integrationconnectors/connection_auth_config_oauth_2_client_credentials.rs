#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionAuthConfigOauth2ClientCredentials {
    /// Secret version of Password for Authentication.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// Secret version reference containing the client secret.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2ClientCredentialsClientSecret>>,
}
