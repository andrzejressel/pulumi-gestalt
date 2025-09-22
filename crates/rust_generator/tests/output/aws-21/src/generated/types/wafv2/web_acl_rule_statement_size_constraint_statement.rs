#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementSizeConstraintStatement {
    /// Operator to use to compare the request part to the size setting. Valid values include: `EQ`, `NE`, `LE`, `LT`, `GE`, or `GT`.
    #[builder(into)]
    #[serde(rename = "comparisonOperator")]
    pub r#comparison_operator: String,
    /// Part of a web request that you want AWS WAF to inspect. See `field_to_match` below for details.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<Option<super::super::types::wafv2::WebAclRuleStatementSizeConstraintStatementFieldToMatch>>,
    /// Size, in bytes, to compare to the request part, after any transformations. Valid values are integers between 0 and 21474836480, inclusive.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: i32,
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. At least one transformation is required. See `text_transformation` below for details.
    #[builder(into)]
    #[serde(rename = "textTransformations")]
    pub r#text_transformations: Vec<super::super::types::wafv2::WebAclRuleStatementSizeConstraintStatementTextTransformation>,
}
