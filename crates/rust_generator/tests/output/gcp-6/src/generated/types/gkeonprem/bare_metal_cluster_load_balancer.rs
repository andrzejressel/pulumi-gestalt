#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalClusterLoadBalancer {
    /// Configuration for BGP typed load balancers.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bgpLbConfig")]
    pub r#bgp_lb_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterLoadBalancerBgpLbConfig>>,
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "manualLbConfig")]
    pub r#manual_lb_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterLoadBalancerManualLbConfig>>,
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "metalLbConfig")]
    pub r#metal_lb_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterLoadBalancerMetalLbConfig>>,
    /// Specifies the load balancer ports.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "portConfig")]
    pub r#port_config: Box<super::super::types::gkeonprem::BareMetalClusterLoadBalancerPortConfig>,
    /// Specified the Bare Metal Load Balancer Config
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vipConfig")]
    pub r#vip_config: Box<super::super::types::gkeonprem::BareMetalClusterLoadBalancerVipConfig>,
}
