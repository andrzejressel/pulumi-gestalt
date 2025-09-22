#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce {
    #[builder(into)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Option<String>,
    /// The secret manager ARN, which contains the client ID and client secret of the connected app.
    #[builder(into)]
    #[serde(rename = "clientCredentialsArn")]
    pub r#client_credentials_arn: Option<String>,
    /// A JSON web token (JWT) that authorizes access to Salesforce records.
    #[builder(into)]
    #[serde(rename = "jwtToken")]
    pub r#jwt_token: Option<String>,
    #[builder(into)]
    #[serde(rename = "oauth2GrantType")]
    pub r#oauth_2_grant_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "oauthRequest")]
    pub r#oauth_request: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforceOauthRequest>>,
    #[builder(into)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Option<String>,
}
