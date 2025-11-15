#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorExplicitResourceOrder {
    #[builder(into)]
    #[serde(rename = "backendPoolHealthProbeIds")]
    pub r#backend_pool_health_probe_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "backendPoolIds")]
    pub r#backend_pool_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "backendPoolLoadBalancingIds")]
    pub r#backend_pool_load_balancing_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "frontendEndpointIds")]
    pub r#frontend_endpoint_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "routingRuleIds")]
    pub r#routing_rule_ids: Option<Vec<String>>,
}
