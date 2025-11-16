#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FindingsFilterFindingCriteriaCriterion {
    /// The value for the property exclusively matches (equals an exact match for) all the specified values. If you specify multiple values, Amazon Macie uses AND logic to join the values.
    #[builder(into)]
    #[serde(rename = "eqExactMatches")]
    pub r#eq_exact_matches: Option<Vec<String>>,
    /// The value for the property matches (equals) the specified value. If you specify multiple values, Amazon Macie uses OR logic to join the values.
    #[builder(into)]
    #[serde(rename = "eqs")]
    pub r#eqs: Option<Vec<String>>,
    /// The name of the field to be evaluated.
    #[builder(into)]
    #[serde(rename = "field")]
    pub r#field: String,
    /// The value for the property is greater than the specified value.
    #[builder(into)]
    #[serde(rename = "gt")]
    pub r#gt: Option<String>,
    /// The value for the property is greater than or equal to the specified value.
    #[builder(into)]
    #[serde(rename = "gte")]
    pub r#gte: Option<String>,
    /// The value for the property is less than the specified value.
    #[builder(into)]
    #[serde(rename = "lt")]
    pub r#lt: Option<String>,
    /// The value for the property is less than or equal to the specified value.
    #[builder(into)]
    #[serde(rename = "lte")]
    pub r#lte: Option<String>,
    /// The value for the property doesn't match (doesn't equal) the specified value. If you specify multiple values, Amazon Macie uses OR logic to join the values.
    #[builder(into)]
    #[serde(rename = "neqs")]
    pub r#neqs: Option<Vec<String>>,
}
