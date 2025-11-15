#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceQuotaUsageMetric {
    /// The metric dimensions.
    #[builder(into)]
    #[serde(rename = "metricDimensions")]
    pub r#metric_dimensions: Vec<super::super::types::servicequotas::GetServiceQuotaUsageMetricMetricDimension>,
    /// The name of the metric.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: String,
    /// The namespace of the metric.
    #[builder(into)]
    #[serde(rename = "metricNamespace")]
    pub r#metric_namespace: String,
    /// The metric statistic that AWS recommend you use when determining quota usage.
    #[builder(into)]
    #[serde(rename = "metricStatisticRecommendation")]
    pub r#metric_statistic_recommendation: String,
}
