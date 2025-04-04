#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatement {
    /// The part of a web request that you want AWS WAF to inspect. See Field to Match below for details.
    #[builder(into, default)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementFieldToMatch>>,
    /// The string representing the regular expression. **Note:** The fixed quota for the maximum number of characters in each regex pattern is 200, which can't be changed. See [AWS WAF quotas](https://docs.aws.amazon.com/waf/latest/developerguide/limits.html) for details.
    #[builder(into)]
    #[serde(rename = "regexString")]
    pub r#regex_string: Box<String>,
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection.
    /// At least one required.
    /// See Text Transformation below for details.
    #[builder(into)]
    #[serde(rename = "textTransformations")]
    pub r#text_transformations: Box<Vec<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatementTextTransformation>>,
}
