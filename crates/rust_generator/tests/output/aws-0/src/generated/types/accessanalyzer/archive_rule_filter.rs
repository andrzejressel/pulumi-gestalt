#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ArchiveRuleFilter {
    /// Contains comparator.
    #[builder(into)]
    #[serde(rename = "contains")]
    pub r#contains: Option<Vec<String>>,
    /// Filter criteria.
    #[builder(into)]
    #[serde(rename = "criteria")]
    pub r#criteria: String,
    /// Equals comparator.
    #[builder(into)]
    #[serde(rename = "eqs")]
    pub r#eqs: Option<Vec<String>>,
    /// Boolean comparator.
    #[builder(into)]
    #[serde(rename = "exists")]
    pub r#exists: Option<String>,
    /// Not Equals comparator.
    #[builder(into)]
    #[serde(rename = "neqs")]
    pub r#neqs: Option<Vec<String>>,
}
