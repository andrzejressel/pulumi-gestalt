#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RulesetRuleActionParametersFromValue {
    /// Preserve query string for redirect URL.
    #[builder(into)]
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Option<bool>,
    /// Status code for redirect.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
    /// Target URL for redirect.
    #[builder(into)]
    #[serde(rename = "targetUrl")]
    pub r#target_url: Option<Box<super::types::RulesetRuleActionParametersFromValueTargetUrl>>,
}
