#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterLoadBalancer {
    /// Configuration for F5 Big IP typed load balancers.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "f5Config")]
    pub r#f_5_config: Option<Box<super::super::types::gkeonprem::VMwareClusterLoadBalancerF5Config>>,
    /// Manually configured load balancers.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "manualLbConfig")]
    pub r#manual_lb_config: Option<Box<super::super::types::gkeonprem::VMwareClusterLoadBalancerManualLbConfig>>,
    /// Configuration for MetalLB typed load balancers.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "metalLbConfig")]
    pub r#metal_lb_config: Option<Box<super::super::types::gkeonprem::VMwareClusterLoadBalancerMetalLbConfig>>,
    /// The VIPs used by the load balancer.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vipConfig")]
    pub r#vip_config: Option<Box<super::super::types::gkeonprem::VMwareClusterLoadBalancerVipConfig>>,
}
