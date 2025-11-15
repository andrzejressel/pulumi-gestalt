#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StandardAppVersionAutomaticScalingStandardSchedulerSettings {
    /// Maximum number of instances to run for this version. Set to zero to disable maxInstances configuration.
    #[builder(into)]
    #[serde(rename = "maxInstances")]
    pub r#max_instances: Option<i32>,
    /// Minimum number of instances to run for this version. Set to zero to disable minInstances configuration.
    #[builder(into)]
    #[serde(rename = "minInstances")]
    pub r#min_instances: Option<i32>,
    /// Target CPU utilization ratio to maintain when scaling. Should be a value in the range [0.50, 0.95], zero, or a negative value.
    #[builder(into)]
    #[serde(rename = "targetCpuUtilization")]
    pub r#target_cpu_utilization: Option<f64>,
    /// Target throughput utilization ratio to maintain when scaling. Should be a value in the range [0.50, 0.95], zero, or a negative value.
    #[builder(into)]
    #[serde(rename = "targetThroughputUtilization")]
    pub r#target_throughput_utilization: Option<f64>,
}
