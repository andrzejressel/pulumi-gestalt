#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementRegexMatchStatement {
    /// The part of a web request that you want AWS WAF to inspect. See `field_to_match` below for details.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementRegexMatchStatementFieldToMatch>>,
    /// String representing the regular expression. Minimum of `1` and maximum of `512` characters.
    #[builder(into)]
    #[serde(rename = "regexString")]
    pub r#regex_string: String,
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. At least one transformation is required. See `text_transformation` below for details.
    #[builder(into)]
    #[serde(rename = "textTransformations")]
    pub r#text_transformations: Vec<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementRegexMatchStatementTextTransformation>,
}
