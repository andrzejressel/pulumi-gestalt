#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnomalySubscriptionThresholdExpressionCostCategory {
    /// Unique name of the Cost Category.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Match options that you can use to filter your results. MatchOptions is only applicable for actions related to cost category. The default values for MatchOptions is `EQUALS` and `CASE_SENSITIVE`. Valid values are: `EQUALS`,  `ABSENT`, `STARTS_WITH`, `ENDS_WITH`, `CONTAINS`, `CASE_SENSITIVE`, `CASE_INSENSITIVE`.
    #[builder(into)]
    #[serde(rename = "matchOptions")]
    pub r#match_options: Option<Vec<String>>,
    /// Specific value of the Cost Category.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Option<Vec<String>>,
}
