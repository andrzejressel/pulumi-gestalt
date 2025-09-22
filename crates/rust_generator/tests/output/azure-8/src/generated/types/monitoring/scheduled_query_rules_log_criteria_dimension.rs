#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScheduledQueryRulesLogCriteriaDimension {
    /// Name of the dimension.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Operator for dimension values, - 'Include'. Defaults to `Include`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Option<String>,
    /// List of dimension values.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
