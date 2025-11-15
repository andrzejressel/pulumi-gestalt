#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRulesetsRulesetRuleLogging {
    /// Override the default logging behavior when a rule is matched.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Override the default logging behavior when a rule is matched. Available values: `enabled`, `disabled`
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
