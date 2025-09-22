#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TargetGroupTargetGroupHealthUnhealthyStateRouting {
    /// The minimum number of targets that must be healthy. If the number of healthy targets is below this value, send traffic to all targets, including unhealthy targets. The possible values are `1` to the maximum number of targets. The default is `1`.
    #[builder(into)]
    #[serde(rename = "minimumHealthyTargetsCount")]
    pub r#minimum_healthy_targets_count: Option<i32>,
    /// The minimum percentage of targets that must be healthy. If the percentage of healthy targets is below this value, send traffic to all targets, including unhealthy targets. The possible values are `off` or an integer from `1` to `100`. The default is `off`.
    #[builder(into)]
    #[serde(rename = "minimumHealthyTargetsPercentage")]
    pub r#minimum_healthy_targets_percentage: Option<String>,
}
