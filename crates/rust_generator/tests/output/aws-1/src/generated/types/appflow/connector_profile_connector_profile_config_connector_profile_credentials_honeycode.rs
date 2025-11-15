#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycode {
    #[builder(into)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Option<String>,
    #[builder(into)]
    #[serde(rename = "oauthRequest")]
    pub r#oauth_request: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycodeOauthRequest>>,
    #[builder(into)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Option<String>,
}
