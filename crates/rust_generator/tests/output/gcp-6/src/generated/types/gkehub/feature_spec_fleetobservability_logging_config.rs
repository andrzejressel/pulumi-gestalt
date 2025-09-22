#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureSpecFleetobservabilityLoggingConfig {
    /// Specified if applying the default routing config to logs not specified in other configs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "defaultConfig")]
    pub r#default_config: Box<Option<super::super::types::gkehub::FeatureSpecFleetobservabilityLoggingConfigDefaultConfig>>,
    /// Specified if applying the routing config to all logs for all fleet scopes.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fleetScopeLogsConfig")]
    pub r#fleet_scope_logs_config: Box<Option<super::super::types::gkehub::FeatureSpecFleetobservabilityLoggingConfigFleetScopeLogsConfig>>,
}
