#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetricAlertCriteriaDimension {
    /// One of the dimension names.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The dimension operator. Possible values are `Include`, `Exclude` and `StartsWith`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// The list of dimension values.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
