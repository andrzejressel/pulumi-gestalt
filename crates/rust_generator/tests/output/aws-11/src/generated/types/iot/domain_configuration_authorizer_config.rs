#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainConfigurationAuthorizerConfig {
    /// A Boolean that specifies whether the domain configuration's authorization service can be overridden.
    #[builder(into)]
    #[serde(rename = "allowAuthorizerOverride")]
    pub r#allow_authorizer_override: Option<bool>,
    /// The name of the authorization service for a domain configuration.
    #[builder(into)]
    #[serde(rename = "defaultAuthorizerName")]
    pub r#default_authorizer_name: Option<String>,
}
