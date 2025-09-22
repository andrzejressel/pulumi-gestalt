#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RulesetRuleActionParametersCacheKeyCustomKey {
    /// Cookie parameters for the custom key.
    #[builder(into)]
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<super::types::RulesetRuleActionParametersCacheKeyCustomKeyCookie>>,
    /// Header parameters for the custom key.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: Box<Option<super::types::RulesetRuleActionParametersCacheKeyCustomKeyHeader>>,
    /// Host parameters for the custom key.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<Option<super::types::RulesetRuleActionParametersCacheKeyCustomKeyHost>>,
    /// Query string parameters for the custom key.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Box<Option<super::types::RulesetRuleActionParametersCacheKeyCustomKeyQueryString>>,
    /// User parameters for the custom key.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Box<Option<super::types::RulesetRuleActionParametersCacheKeyCustomKeyUser>>,
}
