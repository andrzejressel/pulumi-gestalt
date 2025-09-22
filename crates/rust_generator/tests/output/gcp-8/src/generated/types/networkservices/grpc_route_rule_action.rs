#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GrpcRouteRuleAction {
    /// The destination to which traffic should be forwarded.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Option<Vec<super::super::types::networkservices::GrpcRouteRuleActionDestination>>,
    /// The specification for fault injection introduced into traffic to test the resiliency of clients to backend service failure.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "faultInjectionPolicy")]
    pub r#fault_injection_policy: Box<Option<super::super::types::networkservices::GrpcRouteRuleActionFaultInjectionPolicy>>,
    /// Specifies the retry policy associated with this route.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Box<Option<super::super::types::networkservices::GrpcRouteRuleActionRetryPolicy>>,
    /// Specifies the timeout for selected route.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}
