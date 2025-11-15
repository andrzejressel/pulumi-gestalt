#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHeader {
    /// List of headers to check for presence in the custom key.
    #[builder(into)]
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Option<Vec<String>>,
    /// Dictionary of headers mapping to lists of values to check for presence in the custom key.
    #[builder(into)]
    #[serde(rename = "contains")]
    pub r#contains: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Exclude the origin header from the custom key.
    #[builder(into)]
    #[serde(rename = "excludeOrigin")]
    pub r#exclude_origin: Option<bool>,
    /// List of headers to include in the custom key.
    #[builder(into)]
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}
