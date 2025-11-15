#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRulesetsFilter {
    /// The ID of the Ruleset to target.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Type of Ruleset to create. Available values: `custom`, `managed`, `root`, `zone`.
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: Option<String>,
    /// Name of the ruleset.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
    #[builder(into)]
    #[serde(rename = "phase")]
    pub r#phase: Option<String>,
    /// Version of the ruleset to filter on.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
