#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyStepScalingPolicyConfigurationStepAdjustment {
    /// Lower bound for the difference between the alarm threshold and the CloudWatch metric. Without a value, AWS will treat this bound as negative infinity.
    #[builder(into)]
    #[serde(rename = "metricIntervalLowerBound")]
    pub r#metric_interval_lower_bound: Option<String>,
    /// Upper bound for the difference between the alarm threshold and the CloudWatch metric. Without a value, AWS will treat this bound as infinity. The upper bound must be greater than the lower bound.
    #[builder(into)]
    #[serde(rename = "metricIntervalUpperBound")]
    pub r#metric_interval_upper_bound: Option<String>,
    /// Number of members by which to scale, when the adjustment bounds are breached. A positive value scales up. A negative value scales down.
    #[builder(into)]
    #[serde(rename = "scalingAdjustment")]
    pub r#scaling_adjustment: i32,
}
