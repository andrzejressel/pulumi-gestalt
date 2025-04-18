#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ArchiveRuleFilter {
    /// Contains comparator.
    #[builder(into, default)]
    #[serde(rename = "contains")]
    pub r#contains: Box<Option<Vec<String>>>,
    /// Filter criteria.
    #[builder(into)]
    #[serde(rename = "criteria")]
    pub r#criteria: Box<String>,
    /// Equals comparator.
    #[builder(into, default)]
    #[serde(rename = "eqs")]
    pub r#eqs: Box<Option<Vec<String>>>,
    /// Boolean comparator.
    #[builder(into, default)]
    #[serde(rename = "exists")]
    pub r#exists: Box<Option<String>>,
    /// Not Equals comparator.
    #[builder(into, default)]
    #[serde(rename = "neqs")]
    pub r#neqs: Box<Option<Vec<String>>>,
}
