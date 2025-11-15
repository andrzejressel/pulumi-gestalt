#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AddonsConfigAddonsConfig {
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "advancedApiOpsConfig")]
    pub r#advanced_api_ops_config: Option<Box<super::super::types::apigee::AddonsConfigAddonsConfigAdvancedApiOpsConfig>>,
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "apiSecurityConfig")]
    pub r#api_security_config: Option<Box<super::super::types::apigee::AddonsConfigAddonsConfigApiSecurityConfig>>,
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "connectorsPlatformConfig")]
    pub r#connectors_platform_config: Option<Box<super::super::types::apigee::AddonsConfigAddonsConfigConnectorsPlatformConfig>>,
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "integrationConfig")]
    pub r#integration_config: Option<Box<super::super::types::apigee::AddonsConfigAddonsConfigIntegrationConfig>>,
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "monetizationConfig")]
    pub r#monetization_config: Option<Box<super::super::types::apigee::AddonsConfigAddonsConfigMonetizationConfig>>,
}
