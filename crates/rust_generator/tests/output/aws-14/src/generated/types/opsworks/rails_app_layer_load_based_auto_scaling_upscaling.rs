#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RailsAppLayerLoadBasedAutoScalingUpscaling {
    #[builder(into)]
    #[serde(rename = "alarms")]
    pub r#alarms: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "cpuThreshold")]
    pub r#cpu_threshold: Option<f64>,
    #[builder(into)]
    #[serde(rename = "ignoreMetricsTime")]
    pub r#ignore_metrics_time: Option<i32>,
    #[builder(into)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: Option<i32>,
    #[builder(into)]
    #[serde(rename = "loadThreshold")]
    pub r#load_threshold: Option<f64>,
    #[builder(into)]
    #[serde(rename = "memoryThreshold")]
    pub r#memory_threshold: Option<f64>,
    #[builder(into)]
    #[serde(rename = "thresholdsWaitTime")]
    pub r#thresholds_wait_time: Option<i32>,
}
