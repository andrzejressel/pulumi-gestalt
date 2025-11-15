#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfiguration {
    /// Collection of keywords to match.
    #[builder(into)]
    #[serde(rename = "keywords")]
    pub r#keywords: Vec<String>,
    /// Negate the rule.
    #[builder(into)]
    #[serde(rename = "negate")]
    pub r#negate: Option<bool>,
    /// Rule name.
    #[builder(into)]
    #[serde(rename = "ruleName")]
    pub r#rule_name: String,
}
