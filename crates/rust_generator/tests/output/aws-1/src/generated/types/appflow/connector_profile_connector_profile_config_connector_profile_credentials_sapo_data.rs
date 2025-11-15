#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoData {
    /// The SAPOData basic authentication credentials.
    #[builder(into)]
    #[serde(rename = "basicAuthCredentials")]
    pub r#basic_auth_credentials: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataBasicAuthCredentials>>,
    /// The SAPOData OAuth type authentication credentials.
    #[builder(into)]
    #[serde(rename = "oauthCredentials")]
    pub r#oauth_credentials: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentials>>,
}
