#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyPredictiveScalingConfiguration {
    /// Defines the behavior that should be applied if the forecast capacity approaches or exceeds the maximum capacity of the Auto Scaling group. Valid values are `HonorMaxCapacity` or `IncreaseMaxCapacity`. Default is `HonorMaxCapacity`.
    #[builder(into)]
    #[serde(rename = "maxCapacityBreachBehavior")]
    pub r#max_capacity_breach_behavior: Option<String>,
    /// Size of the capacity buffer to use when the forecast capacity is close to or exceeds the maximum capacity. Valid range is `0` to `100`. If set to `0`, Amazon EC2 Auto Scaling may scale capacity higher than the maximum capacity to equal but not exceed forecast capacity.
    #[builder(into)]
    #[serde(rename = "maxCapacityBuffer")]
    pub r#max_capacity_buffer: Option<String>,
    /// This structure includes the metrics and target utilization to use for predictive scaling.
    #[builder(into)]
    #[serde(rename = "metricSpecification")]
    pub r#metric_specification: Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecification>,
    /// Predictive scaling mode. Valid values are `ForecastAndScale` and `ForecastOnly`. Default is `ForecastOnly`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// Amount of time, in seconds, by which the instance launch time can be advanced. Minimum is `0`.
    #[builder(into)]
    #[serde(rename = "schedulingBufferTime")]
    pub r#scheduling_buffer_time: Option<String>,
}
