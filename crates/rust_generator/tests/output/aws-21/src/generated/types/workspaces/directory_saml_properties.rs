#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DirectorySamlProperties {
    /// The relay state parameter name supported by the SAML 2.0 identity provider (IdP). Default `RelayState`.
    #[builder(into)]
    #[serde(rename = "relayStateParameterName")]
    pub r#relay_state_parameter_name: Option<String>,
    /// Status of SAML 2.0 authentication. Default `DISABLED`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// The SAML 2.0 identity provider (IdP) user access URL.
    #[builder(into)]
    #[serde(rename = "userAccessUrl")]
    pub r#user_access_url: Option<String>,
}
