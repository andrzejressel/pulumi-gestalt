#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    /// List of query string parameters to exclude from the custom key. Conflicts with "include".
    #[builder(into)]
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Vec<String>>,
    /// List of query string parameters to include in the custom key. Conflicts with "exclude".
    #[builder(into)]
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}
