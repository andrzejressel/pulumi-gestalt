#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData {
    /// The location of the SAPOData resource.
    #[builder(into)]
    #[serde(rename = "applicationHostUrl")]
    pub r#application_host_url: String,
    /// The application path to catalog service.
    #[builder(into)]
    #[serde(rename = "applicationServicePath")]
    pub r#application_service_path: String,
    /// The client number for the client creating the connection.
    #[builder(into)]
    #[serde(rename = "clientNumber")]
    pub r#client_number: String,
    /// The logon language of SAPOData instance.
    #[builder(into)]
    #[serde(rename = "logonLanguage")]
    pub r#logon_language: Option<String>,
    /// The SAPOData OAuth properties required for OAuth type authentication.
    #[builder(into)]
    #[serde(rename = "oauthProperties")]
    pub r#oauth_properties: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoDataOauthProperties>>,
    /// The port number of the SAPOData instance.
    #[builder(into)]
    #[serde(rename = "portNumber")]
    pub r#port_number: i32,
    #[builder(into)]
    #[serde(rename = "privateLinkServiceName")]
    pub r#private_link_service_name: Option<String>,
}
