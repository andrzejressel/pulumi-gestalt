#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecificationMetricDataQuery {
    /// Math expression used on the returned metric. You must specify either `expression` or `metric_stat`, but not both.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    /// Short name for the metric used in predictive scaling policy.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Human-readable label for this metric or expression.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// Structure that defines CloudWatch metric to be used in predictive scaling policy. You must specify either `expression` or `metric_stat`, but not both.
    #[builder(into)]
    #[serde(rename = "metricStat")]
    pub r#metric_stat: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecificationMetricDataQueryMetricStat>>,
    /// Boolean that indicates whether to return the timestamps and raw data values of this metric, the default is true
    #[builder(into)]
    #[serde(rename = "returnData")]
    pub r#return_data: Option<bool>,
}
