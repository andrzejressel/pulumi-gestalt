#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RulesetRuleActionParametersEdgeTtlStatusCodeTtl {
    /// Status code for which the edge TTL is applied.
    #[builder(into, default)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Status code range for which the edge TTL is applied.
    #[builder(into, default)]
    #[serde(rename = "statusCodeRanges")]
    pub r#status_code_ranges: Box<Option<Vec<super::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange>>>,
    /// Status code edge TTL value.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<i32>>,
}
