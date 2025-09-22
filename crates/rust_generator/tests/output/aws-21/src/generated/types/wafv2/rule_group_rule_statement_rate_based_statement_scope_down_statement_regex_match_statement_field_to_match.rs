#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatch {
    /// Inspect all query arguments.
    #[builder(into)]
    #[serde(rename = "allQueryArguments")]
    pub r#all_query_arguments: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatchAllQueryArguments>>,
    /// Inspect the request body, which immediately follows the request headers.
    #[builder(into)]
    #[serde(rename = "body")]
    pub r#body: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatchBody>>,
    /// Inspect the cookies in the web request. See Cookies below for details.
    #[builder(into)]
    #[serde(rename = "cookies")]
    pub r#cookies: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatchCookies>>,
    /// Inspect the request headers. See Header Order below for details.
    #[builder(into)]
    #[serde(rename = "headerOrders")]
    pub r#header_orders: Option<Vec<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatchHeaderOrder>>,
    /// Inspect the request headers. See Headers below for details.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatchHeader>>,
    #[builder(into)]
    #[serde(rename = "ja3Fingerprint")]
    pub r#ja_3_fingerprint: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatchJa3Fingerprint>>,
    /// Inspect the request body as JSON. See JSON Body for details.
    #[builder(into)]
    #[serde(rename = "jsonBody")]
    pub r#json_body: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatchJsonBody>>,
    /// Inspect the HTTP method. The method indicates the type of operation that the request is asking the origin to perform.
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatchMethod>>,
    /// Inspect the query string. This is the part of a URL that appears after a `?` character, if any.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatchQueryString>>,
    /// Inspect a single header. See Single Header below for details.
    #[builder(into)]
    #[serde(rename = "singleHeader")]
    pub r#single_header: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatchSingleHeader>>,
    /// Inspect a single query argument. See Single Query Argument below for details.
    #[builder(into)]
    #[serde(rename = "singleQueryArgument")]
    pub r#single_query_argument: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatchSingleQueryArgument>>,
    /// Inspect the request URI path. This is the part of a web request that identifies a resource, for example, `/images/daily-ad.jpg`.
    #[builder(into)]
    #[serde(rename = "uriPath")]
    pub r#uri_path: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatchUriPath>>,
}
