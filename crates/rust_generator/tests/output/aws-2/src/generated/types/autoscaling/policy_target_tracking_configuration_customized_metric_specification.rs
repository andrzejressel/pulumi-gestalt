#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyTargetTrackingConfigurationCustomizedMetricSpecification {
    /// Dimensions of the metric.
    #[builder(into)]
    #[serde(rename = "metricDimensions")]
    pub r#metric_dimensions: Option<Vec<super::super::types::autoscaling::PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetricDimension>>,
    /// Name of the metric.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Option<String>,
    /// Metrics to include, as a metric data query.
    #[builder(into)]
    #[serde(rename = "metrics")]
    pub r#metrics: Option<Vec<super::super::types::autoscaling::PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetric>>,
    /// Namespace of the metric.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Option<String>,
    /// Statistic of the metric.
    #[builder(into)]
    #[serde(rename = "statistic")]
    pub r#statistic: Option<String>,
    /// Unit of the metric.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Option<String>,
}
