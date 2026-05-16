#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustGatewaySettingsLoggingSettingsByRuleType {
    /// Logging configuration for DNS requests.
    #[builder(into)]
    #[serde(rename = "dns")]
    pub r#dns: Box<super::types::ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeDns>,
    /// Logging configuration for HTTP requests.
    #[builder(into)]
    #[serde(rename = "http")]
    pub r#http: Box<super::types::ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeHttp>,
    /// Logging configuration for layer 4 requests.
    #[builder(into)]
    #[serde(rename = "l4")]
    pub r#l_4: Box<super::types::ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeL4>,
}
