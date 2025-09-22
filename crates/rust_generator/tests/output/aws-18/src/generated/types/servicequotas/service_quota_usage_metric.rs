#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceQuotaUsageMetric {
    /// The metric dimensions.
    #[builder(into)]
    #[serde(rename = "metricDimensions")]
    pub r#metric_dimensions: Option<Vec<super::super::types::servicequotas::ServiceQuotaUsageMetricMetricDimension>>,
    /// The name of the metric.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Option<String>,
    /// The namespace of the metric.
    #[builder(into)]
    #[serde(rename = "metricNamespace")]
    pub r#metric_namespace: Option<String>,
    /// The metric statistic that AWS recommend you use when determining quota usage.
    #[builder(into)]
    #[serde(rename = "metricStatisticRecommendation")]
    pub r#metric_statistic_recommendation: Option<String>,
}
