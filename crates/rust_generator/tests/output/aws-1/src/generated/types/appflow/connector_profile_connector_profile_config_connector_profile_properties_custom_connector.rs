#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector {
    /// The OAuth 2.0 properties required for OAuth 2.0 authentication.
    #[builder(into)]
    #[serde(rename = "oauth2Properties")]
    pub r#oauth_2_properties: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnectorOauth2Properties>>,
    /// A map of properties that are required to create a profile for the custom connector.
    #[builder(into)]
    #[serde(rename = "profileProperties")]
    pub r#profile_properties: Option<std::collections::HashMap<String, String>>,
}
