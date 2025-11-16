#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceAutoscalingConfig {
    /// Asymmetric autoscaling options for specific replicas.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "asymmetricAutoscalingOptions")]
    pub r#asymmetric_autoscaling_options: Option<Vec<super::super::types::spanner::InstanceAutoscalingConfigAsymmetricAutoscalingOption>>,
    /// Defines scale in controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events. Users can define the minimum and
    /// maximum compute capacity allocated to the instance, and the autoscaler will
    /// only scale within that range. Users can either use nodes or processing
    /// units to specify the limits, but should use the same unit to set both the
    /// min_limit and max_limit.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "autoscalingLimits")]
    pub r#autoscaling_limits: Option<Box<super::super::types::spanner::InstanceAutoscalingConfigAutoscalingLimits>>,
    /// Defines scale in controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "autoscalingTargets")]
    pub r#autoscaling_targets: Option<Box<super::super::types::spanner::InstanceAutoscalingConfigAutoscalingTargets>>,
}
