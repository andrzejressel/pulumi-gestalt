#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutoscalerAutoscalingPolicy {
    /// The number of seconds that the autoscaler should wait before it
    /// starts collecting information from a new instance. This prevents
    /// the autoscaler from collecting information when the instance is
    /// initializing, during which the collected usage would not be
    /// reliable. The default time autoscaler waits is 60 seconds.
    /// Virtual machine initialization times might vary because of
    /// numerous factors. We recommend that you test how long an
    /// instance may take to initialize. To do this, create an instance
    /// and time the startup process.
    #[builder(into)]
    #[serde(rename = "cooldownPeriod")]
    pub r#cooldown_period: Option<i32>,
    /// Defines the CPU utilization policy that allows the autoscaler to
    /// scale based on the average CPU utilization of a managed instance
    /// group.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cpuUtilization")]
    pub r#cpu_utilization: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyCpuUtilization>>,
    /// Configuration parameters of autoscaling based on a load balancer.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "loadBalancingUtilization")]
    pub r#load_balancing_utilization: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyLoadBalancingUtilization>>,
    /// The maximum number of instances that the autoscaler can scale up
    /// to. This is required when creating or updating an autoscaler. The
    /// maximum number of replicas should not be lower than minimal number
    /// of replicas.
    #[builder(into)]
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: i32,
    /// Configuration parameters of autoscaling based on a custom metric.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "metrics")]
    pub r#metrics: Option<Vec<super::super::types::compute::AutoscalerAutoscalingPolicyMetric>>,
    /// The minimum number of replicas that the autoscaler can scale down
    /// to. This cannot be less than 0. If not provided, autoscaler will
    /// choose a default value depending on maximum number of instances
    /// allowed.
    #[builder(into)]
    #[serde(rename = "minReplicas")]
    pub r#min_replicas: i32,
    /// Defines operating mode for this policy.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// Defines scale down controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "scaleDownControl")]
    pub r#scale_down_control: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyScaleDownControl>>,
    /// Defines scale in controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "scaleInControl")]
    pub r#scale_in_control: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyScaleInControl>>,
    /// Scaling schedules defined for an autoscaler. Multiple schedules can be set on an autoscaler and they can overlap.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "scalingSchedules")]
    pub r#scaling_schedules: Option<Vec<super::super::types::compute::AutoscalerAutoscalingPolicyScalingSchedule>>,
}
