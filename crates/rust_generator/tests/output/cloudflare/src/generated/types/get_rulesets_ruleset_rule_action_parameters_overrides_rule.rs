#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRulesetsRulesetRuleActionParametersOverridesRule {
    /// Action to perform in the rule-level override. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    /// Defines if the current rule-level override enables or disables the rule.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The ID of this resource.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Anomaly score threshold to apply in the ruleset rule override. Only applicable to modsecurity-based rulesets.
    #[builder(into)]
    #[serde(rename = "scoreThreshold")]
    pub r#score_threshold: Option<i32>,
    /// Sensitivity level for a ruleset rule override.
    #[builder(into)]
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Option<String>,
    /// Defines if the current rule-level override enables or disables the rule. Available values: `enabled`, `disabled`
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
