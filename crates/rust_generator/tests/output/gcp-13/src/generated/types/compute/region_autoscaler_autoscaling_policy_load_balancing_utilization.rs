#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionAutoscalerAutoscalingPolicyLoadBalancingUtilization {
    /// Fraction of backend capacity utilization (set in HTTP(s) load
    /// balancing configuration) that autoscaler should maintain. Must
    /// be a positive float value. If not defined, the default is 0.8.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: f64,
}
