#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnector {
    #[builder(into)]
    #[serde(rename = "apiKey")]
    pub r#api_key: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorApiKey>>,
    /// The authentication type that the custom connector uses for authenticating while creating a connector profile. One of: `APIKEY`, `BASIC`, `CUSTOM`, `OAUTH2`.
    #[builder(into)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: String,
    /// Basic credentials that are required for the authentication of the user.
    #[builder(into)]
    #[serde(rename = "basic")]
    pub r#basic: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorBasic>>,
    /// If the connector uses the custom authentication mechanism, this holds the required credentials.
    #[builder(into)]
    #[serde(rename = "custom")]
    pub r#custom: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorCustom>>,
    /// OAuth 2.0 credentials required for the authentication of the user.
    #[builder(into)]
    #[serde(rename = "oauth2")]
    pub r#oauth_2: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2>>,
}
