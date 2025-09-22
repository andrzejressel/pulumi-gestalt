#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCostCategoryRuleRuleAndOrCostCategory {
    /// Key for the tag.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// Match options that you can use to filter your results. MatchOptions is only applicable for actions related to cost category. The default values for MatchOptions is `EQUALS` and `CASE_SENSITIVE`. Valid values are: `EQUALS`,  `ABSENT`, `STARTS_WITH`, `ENDS_WITH`, `CONTAINS`, `CASE_SENSITIVE`, `CASE_INSENSITIVE`.
    #[builder(into)]
    #[serde(rename = "matchOptions")]
    pub r#match_options: Vec<String>,
    /// Parameter values.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
