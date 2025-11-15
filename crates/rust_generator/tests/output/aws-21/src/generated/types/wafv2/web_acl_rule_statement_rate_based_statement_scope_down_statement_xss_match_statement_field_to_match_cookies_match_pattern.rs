#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementRateBasedStatementScopeDownStatementXssMatchStatementFieldToMatchCookiesMatchPattern {
    /// An empty configuration block that is used for inspecting all headers.
    #[builder(into)]
    #[serde(rename = "all")]
    pub r#all: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementScopeDownStatementXssMatchStatementFieldToMatchCookiesMatchPatternAll>>,
    #[builder(into)]
    #[serde(rename = "excludedCookies")]
    pub r#excluded_cookies: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "includedCookies")]
    pub r#included_cookies: Option<Vec<String>>,
}
