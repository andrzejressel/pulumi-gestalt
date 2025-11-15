#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetricStreamIncludeFilter {
    /// An array that defines the metrics you want to include for this metric namespace
    #[builder(into)]
    #[serde(rename = "metricNames")]
    pub r#metric_names: Option<Vec<String>>,
    /// Name of the metric namespace in the filter.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: String,
}
