#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InsightFiltersCriticality {
    /// The equal-to condition to be applied to a single field when querying for findings, provided as a String.
    #[builder(into)]
    #[serde(rename = "eq")]
    pub r#eq: Option<String>,
    /// The greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.
    #[builder(into)]
    #[serde(rename = "gte")]
    pub r#gte: Option<String>,
    /// The less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
    #[builder(into)]
    #[serde(rename = "lte")]
    pub r#lte: Option<String>,
}
