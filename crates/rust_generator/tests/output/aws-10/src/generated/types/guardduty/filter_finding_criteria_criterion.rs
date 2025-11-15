#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FilterFindingCriteriaCriterion {
    /// List of string values to be evaluated.
    #[builder(into)]
    #[serde(rename = "equals")]
    pub r#equals: Option<Vec<String>>,
    /// The name of the field to be evaluated. The full list of field names can be found in [AWS documentation](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_filter-findings.html#filter_criteria).
    #[builder(into)]
    #[serde(rename = "field")]
    pub r#field: String,
    /// A value to be evaluated. Accepts either an integer or a date in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into)]
    #[serde(rename = "greaterThan")]
    pub r#greater_than: Option<String>,
    /// A value to be evaluated. Accepts either an integer or a date in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into)]
    #[serde(rename = "greaterThanOrEqual")]
    pub r#greater_than_or_equal: Option<String>,
    /// A value to be evaluated. Accepts either an integer or a date in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into)]
    #[serde(rename = "lessThan")]
    pub r#less_than: Option<String>,
    /// A value to be evaluated. Accepts either an integer or a date in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into)]
    #[serde(rename = "lessThanOrEqual")]
    pub r#less_than_or_equal: Option<String>,
    /// List of string values to be evaluated.
    #[builder(into)]
    #[serde(rename = "notEquals")]
    pub r#not_equals: Option<Vec<String>>,
}
