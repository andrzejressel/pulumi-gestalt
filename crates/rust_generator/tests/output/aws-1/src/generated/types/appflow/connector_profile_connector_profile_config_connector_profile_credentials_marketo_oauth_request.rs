#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketoOauthRequest {
    /// The code provided by the connector when it has been authenticated via the connected app.
    #[builder(into)]
    #[serde(rename = "authCode")]
    pub r#auth_code: Option<String>,
    /// The URL to which the authentication server redirects the browser after authorization has been granted.
    #[builder(into)]
    #[serde(rename = "redirectUri")]
    pub r#redirect_uri: Option<String>,
}
