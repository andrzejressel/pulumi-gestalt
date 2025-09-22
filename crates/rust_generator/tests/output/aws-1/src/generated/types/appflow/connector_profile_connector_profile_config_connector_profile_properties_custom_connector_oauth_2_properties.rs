#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnectorOauth2Properties {
    #[builder(into)]
    #[serde(rename = "oauth2GrantType")]
    pub r#oauth_2_grant_type: String,
    #[builder(into)]
    #[serde(rename = "tokenUrl")]
    pub r#token_url: String,
    /// Associates your token URL with a map of properties that you define. Use this parameter to provide any additional details that the connector requires to authenticate your request.
    #[builder(into)]
    #[serde(rename = "tokenUrlCustomProperties")]
    pub r#token_url_custom_properties: Option<std::collections::HashMap<String, String>>,
}
