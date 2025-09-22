#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyTargetTrackingConfigurationPredefinedMetricSpecification {
    /// Metric type.
    #[builder(into)]
    #[serde(rename = "predefinedMetricType")]
    pub r#predefined_metric_type: String,
    /// Identifies the resource associated with the metric type.
    #[builder(into)]
    #[serde(rename = "resourceLabel")]
    pub r#resource_label: Option<String>,
}
