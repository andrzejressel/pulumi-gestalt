#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKey {
    /// Cookie parameters for the custom key.
    #[builder(into)]
    #[serde(rename = "cookie")]
    pub r#cookie: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyCookie>>,
    /// Header parameters for the custom key.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHeader>>,
    /// Host parameters for the custom key.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHost>>,
    /// Query string parameters for the custom key.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString>>,
    /// User parameters for the custom key.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser>>,
}
