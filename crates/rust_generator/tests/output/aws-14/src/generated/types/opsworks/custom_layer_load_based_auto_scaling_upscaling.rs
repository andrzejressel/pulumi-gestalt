#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomLayerLoadBasedAutoScalingUpscaling {
    /// Custom Cloudwatch auto scaling alarms, to be used as thresholds. This parameter takes a list of up to five alarm names, which are case sensitive and must be in the same region as the stack.
    #[builder(into)]
    #[serde(rename = "alarms")]
    pub r#alarms: Option<Vec<String>>,
    /// The CPU utilization threshold, as a percent of the available CPU. A value of -1 disables the threshold.
    #[builder(into)]
    #[serde(rename = "cpuThreshold")]
    pub r#cpu_threshold: Option<f64>,
    /// The amount of time (in minutes) after a scaling event occurs that AWS OpsWorks Stacks should ignore metrics and suppress additional scaling events.
    #[builder(into)]
    #[serde(rename = "ignoreMetricsTime")]
    pub r#ignore_metrics_time: Option<i32>,
    /// The number of instances to add or remove when the load exceeds a threshold.
    #[builder(into)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: Option<i32>,
    /// The load threshold. A value of -1 disables the threshold.
    #[builder(into)]
    #[serde(rename = "loadThreshold")]
    pub r#load_threshold: Option<f64>,
    /// The memory utilization threshold, as a percent of the available memory. A value of -1 disables the threshold.
    #[builder(into)]
    #[serde(rename = "memoryThreshold")]
    pub r#memory_threshold: Option<f64>,
    /// The amount of time, in minutes, that the load must exceed a threshold before more instances are added or removed.
    #[builder(into)]
    #[serde(rename = "thresholdsWaitTime")]
    pub r#thresholds_wait_time: Option<i32>,
}
